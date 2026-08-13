use std::{cmp::min, collections::HashMap, fmt::Debug};

use tracing::trace;

use crate::{
    bit_reader::BitReader,
    error::EnactError,
    lzx::decoder::LZXDecoder,
    models::{xnb_file_header::XNBFileHeader, xnb_texture2d::XNBTexture2D},
    reader::Reader,
};

pub struct XNBFile {
    header: XNBFileHeader,
    data: Vec<u8>,
    // types: [trait vec]
    // shared_resources: [trait vec]
}

impl Debug for XNBFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XNBFile")
            .field("header", &self.header)
            .field("data", &format_args!("{} bytes", self.data.len()))
            .finish()
    }
}

impl XNBFile {
    #[tracing::instrument(skip_all, fields(offset = reader.pos(), err))]
    pub fn from_reader(reader: &mut Reader) -> Result<Self, EnactError> {
        let header = XNBFileHeader::from_reader(reader)?;

        trace!(?header, "parsed header");

        let total = header.decompressed_size as usize;
        let mut decoder = LZXDecoder::new(16, total)?;

        while decoder.pos() < total {
            let first = reader.u8()?;
            let (frame_size, block_size) = if first == 0xFF {
                let frame = reader.u16_be()? as usize;
                let block = reader.u16_be()? as usize;

                (frame, block)
            } else {
                let second = reader.u8()? as usize;

                (0x8000usize, ((first as usize) << 8) | second)
            };

            trace!("chunk: {} compressed -> {} out", block_size, frame_size);

            let chunk = reader.bytes(block_size)?;
            let mut br = BitReader::new(Reader::new(chunk));
            decoder.decode_frame(&mut br, min(frame_size, total - decoder.pos()))?;
        }

        let out = decoder.into_output();
        let mut out_reader = Reader::new(&out);

        let type_reader_count = out_reader.encoded_int_7bit()? as usize;
        let mut type_readers = HashMap::new();

        for reader_num in 0..type_reader_count {
            let reader_name = out_reader.string()?;
            let _version = out_reader.u32_le()?;

            type_readers.insert(reader_num, reader_name);
        }

        let shared_resources = out_reader.encoded_int_7bit()? as usize;
        let primary_asset_id = out_reader.encoded_int_7bit()? as usize;

        let asset_reader = type_readers
            .get(&(primary_asset_id - 1))
            .ok_or(EnactError::NoTypeReader)?;

        trace!(?asset_reader, ?primary_asset_id);

        let image = XNBTexture2D::from_reader(&mut out_reader)?;
        trace!(?image);

        Ok(Self { header, data: out })
    }
}

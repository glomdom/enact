use crate::{
    error::EnactError,
    lzx::decoder::LZXDecoder,
    models::xnb_file_header::XNBFileHeader,
    readers::{bit_reader::BitReader, byte_reader::ByteReader},
};

use std::{cmp::min, fmt::Debug};
use tracing::trace;

pub struct XNBFile {
    pub header: XNBFileHeader,
    pub data: Vec<u8>,
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
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self, EnactError> {
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
            let mut br = BitReader::new(ByteReader::new(chunk));
            decoder.decode_frame(&mut br, min(frame_size, total - decoder.pos()))?;
        }

        let out = decoder.into_output();

        Ok(Self { header, data: out })
    }
}

use tracing::trace;

use crate::{
    error::EnactError,
    models::{xnb_flag_bits::XNBFlagBits, xnb_target_platform::XNBTargetPlatform},
    readers::byte_reader::ByteReader,
};

#[derive(Debug)]
pub struct XNBFileHeader {
    pub target_platform: XNBTargetPlatform,
    pub format_version: u8,
    pub flag_bits: XNBFlagBits,
    pub compressed_size: u32,
    pub decompressed_size: u32,
}

impl XNBFileHeader {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self, EnactError> {
        let magic = reader.bytes(3)?;
        if magic != b"XNB" {
            return Err(EnactError::BadMagic);
        }

        trace!("XNB magic matches");

        let target_platform = match reader.u8()? {
            b'w' => XNBTargetPlatform::Windows,
            b'm' => XNBTargetPlatform::WindowsPhone,
            b'x' => XNBTargetPlatform::Xbox,

            byte => {
                return Err(EnactError::BadTargetPlatform { byte: byte });
            }
        };

        trace!("target platform is {:?}", target_platform);

        let format_version = reader.u8()?;
        if format_version != 5 {
            return Err(EnactError::BadFormatVersion {
                version: format_version,
            });
        }

        trace!("format version is {}", format_version);

        let flags = reader.u8()?;
        let flag_bits = XNBFlagBits::from_bits(flags).ok_or(EnactError::BadFlagBits)?;

        trace!(?flags, "flag bits are {:?}", flag_bits);

        let compressed_size = reader.u32_le()?;
        let decompressed_size = reader.u32_le()?;

        Ok(Self {
            target_platform,
            format_version,
            flag_bits,
            compressed_size,
            decompressed_size,
        })
    }
}

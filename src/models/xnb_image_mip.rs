use std::fmt::Debug;

use crate::{error::EnactError, readers::byte_reader::ByteReader};

pub struct XNBImageMIP {
    pub size: u32,
    pub data: Vec<u8>,
}

impl Debug for XNBImageMIP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XNBImageMip")
            .field("size", &self.size)
            .field("data", &format_args!("{} bytes", self.data.len()))
            .finish()
    }
}

impl XNBImageMIP {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self, EnactError> {
        let size = reader.u32_le()?;
        let data = reader.bytes(size as usize)?;

        Ok(Self {
            size,
            data: data.to_vec(),
        })
    }
}

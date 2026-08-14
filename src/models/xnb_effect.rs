use crate::{error::EnactError, readers::byte_reader::ByteReader};
use std::fmt::Debug;

pub struct XNBEffect {
    pub size: u32,
    pub bytecode: Vec<u8>,
}

impl Debug for XNBEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XNBEffect")
            .field("size", &self.size)
            .field("bytecode", &format_args!("{} bytes", self.bytecode.len()))
            .finish()
    }
}

impl XNBEffect {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self, EnactError> {
        let size = reader.u32_le()?;
        let bytecode = reader.bytes(size as usize)?;

        Ok(Self {
            size,
            bytecode: bytecode.to_vec(),
        })
    }
}

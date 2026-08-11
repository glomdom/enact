use crate::{bit_reader::BitReader, error::EnactError};

#[derive(Clone, Copy)]
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn with_bits<T>(
        &mut self,
        f: impl FnOnce(&mut BitReader<'a>) -> Result<T, EnactError>,
    ) -> Result<T, EnactError> {
        let mut bit_reader = BitReader::new(*self);
        let out = f(&mut bit_reader)?;

        *self = bit_reader.into_inner()?;

        Ok(out)
    }

    pub fn u8(&mut self) -> Result<u8, EnactError> {
        Ok(self.take(1)?[0])
    }

    pub fn u32_le(&mut self) -> Result<u32, EnactError> {
        let bytes = self.take(4)?;

        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn u16_le(&mut self) -> Result<u16, EnactError> {
        let bytes = self.take(2)?;

        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn u16_be(&mut self) -> Result<u16, EnactError> {
        let bytes = self.take(2)?;

        Ok(u16::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8], EnactError> {
        self.take(n)
    }

    pub fn seek(&mut self, at: usize) -> Result<(), EnactError> {
        if at > self.buf.len() {
            return Err(EnactError::Eof { at, want: 0 });
        }

        self.pos = at;

        Ok(())
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], EnactError> {
        let end = self.pos.checked_add(n).ok_or(EnactError::Overflow)?;
        let s = self.buf.get(self.pos..end).ok_or(EnactError::Eof {
            at: self.pos,
            want: n,
        })?;

        self.pos = end;

        Ok(s)
    }
}

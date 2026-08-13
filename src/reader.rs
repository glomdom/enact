use crate::error::EnactError;

#[derive(Clone, Copy)]
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

/// Specialized reader for reading XNB data
impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn encoded_int_7bit(&mut self) -> Result<i32, EnactError> {
        let mut result = 0;
        let mut bits_read = 0;
        let mut value;

        loop {
            if bits_read == 35 {
                return Err(EnactError::Overflow);
            }

            value = self.take(1)?[0] as i32;
            result |= (value & 0x7F) << bits_read;
            bits_read += 7;

            if value & 0x80 == 0 {
                break;
            }
        }

        Ok(result)
    }

    pub fn string(&mut self) -> Result<String, EnactError> {
        let string_size = self.encoded_int_7bit()?;
        let string_bytes = self.bytes(string_size as usize)?;

        Ok(String::from_utf8_lossy(string_bytes).into_owned())
    }

    pub fn u8(&mut self) -> Result<u8, EnactError> {
        Ok(self.take(1)?[0])
    }

    pub fn u32_le(&mut self) -> Result<u32, EnactError> {
        let bytes = self.take(4)?;

        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn i32_le(&mut self) -> Result<i32, EnactError> {
        let bytes = self.take(4)?;

        Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
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

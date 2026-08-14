use crate::error::EnactError;
use crate::readers::byte_reader::ByteReader;

pub struct BitReader<'a> {
    inner: ByteReader<'a>,
    buf: u32,
    n: u32,
    pad: u32,
}

impl<'a> BitReader<'a> {
    pub fn new(inner: ByteReader<'a>) -> Self {
        Self {
            inner,
            buf: 0,
            n: 0,
            pad: 0,
        }
    }

    fn fill(&mut self, want: u32) -> Result<(), EnactError> {
        debug_assert!(want <= 17);

        while self.n < want {
            let w = match self.inner.u16_le() {
                Ok(w) => w,
                Err(EnactError::Eof { .. }) => {
                    self.pad += 1;

                    0
                }

                Err(e) => return Err(e),
            };

            self.buf |= (w as u32) << (16 - self.n);
            self.n += 16;
        }

        Ok(())
    }

    pub fn peek(&mut self, n: u32) -> Result<u32, EnactError> {
        if n == 0 {
            return Ok(0);
        }

        self.fill(n)?;

        Ok(self.buf >> (32 - n))
    }

    pub fn consume(&mut self, n: u32) {
        self.buf <<= n;
        self.n -= n;
    }

    pub fn bits(&mut self, n: u32) -> Result<u32, EnactError> {
        let v = self.peek(n)?;
        self.consume(n);

        Ok(v)
    }

    /// Discard padding up to the next 16bit boundary.
    pub fn align(&mut self) {
        let rem = self.n % 16;

        self.buf <<= rem;
        self.n -= rem;
    }
}

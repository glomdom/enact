use tracing::trace;

use crate::{error::EnactError, readers::bit_reader::BitReader};

#[derive(Debug)]
pub struct Tree {
    pub lens: Box<[u8]>,
    pub codes: Box<[u32]>,
}

impl Tree {
    pub fn decode(&self, br: &mut BitReader) -> Result<u16, EnactError> {
        let mut accum = 0;
        for bit_count in 1..=16 {
            let bit = br.bits(1)?;
            accum = (accum << 1) | bit;

            for sym in 0..self.lens.len() {
                let len = self.lens[sym];
                let code = self.codes[sym];

                if len == bit_count && code == accum {
                    return Ok(sym as u16);
                }
            }
        }

        Err(EnactError::CorruptedStream)
    }

    pub fn from_lengths(lens: &[u8]) -> Self {
        let mut count = [0u32; 17];
        for &temp in lens {
            if temp == 0 {
                continue;
            }

            count[temp as usize] += 1;
        }

        let mut next_code = [0u32; 17];
        let mut code = 0;
        for len in 1..=16 {
            code = (code + count[len - 1]) << 1;
            next_code[len] = code
        }

        let mut codes = vec![0u32; lens.len()];
        for sym in 0..codes.len() {
            let len = lens[sym] as usize;

            if len != 0 {
                codes[sym] = next_code[len];
                next_code[len] += 1;
            }
        }

        Self {
            lens: lens.to_vec().into_boxed_slice(),
            codes: codes.into(),
        }
    }

    pub fn read_lengths(
        br: &mut BitReader,
        target: &mut [u8],
        start: usize,
        end: usize,
    ) -> Result<(), EnactError> {
        let mut lens: [u8; 20] = [0; 20];
        for i in 0..20 {
            let val = br.bits(4)? as u8;

            lens[i] = val;
        }

        let pretree = Tree::from_lengths(&lens);

        let mut cursor = 0;
        while cursor < end - start {
            let sym = pretree.decode(br)? as u16;
            let idx = start + cursor;

            match sym {
                0..=16 => {
                    let val = (target[start + cursor] as u16 + 17 - sym) % 17;

                    target[idx] = val as u8;
                    cursor += 1
                }

                17 => {
                    let n = br.bits(4)? + 4;

                    for i in 0..n {
                        target[idx + i as usize] = 0;
                    }

                    cursor += n as usize;
                }

                18 => {
                    let n = br.bits(5)? + 20;

                    for i in 0..n {
                        target[idx + i as usize] = 0;
                    }

                    cursor += n as usize;
                }

                19 => {
                    let n = br.bits(1)? + 4;

                    let off = pretree.decode(br)? as u8;
                    let len = (target[idx] + 17 - off) % 17;

                    for i in 0..n {
                        target[idx + i as usize] = len;
                    }

                    cursor += n as usize;
                }

                _ => {
                    trace!("should not happen")
                }
            }
        }

        Ok(())
    }
}

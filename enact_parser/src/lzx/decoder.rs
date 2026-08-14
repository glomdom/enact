use crate::{error::EnactError, lzx::tree::Tree, readers::bit_reader::BitReader};
use std::cmp::min;
use tracing::trace;

const EXTRA_BITS: [u8; 51] = {
    let mut t = [0u8; 51];
    let mut i = 2;

    while i < 51 {
        let v = i / 2 - 1;
        t[i] = if v > 17 { 17 } else { v as u8 };
        i += 1;
    }

    t
};

const POSITION_BASE: [u32; 51] = {
    let mut t = [0u32; 51];
    let mut acc = 0u32;
    let mut i = 0;

    while i < 51 {
        t[i] = acc;
        acc += 1u32 << EXTRA_BITS[i];

        i += 1;
    }

    t
};

pub struct LZXDecoder {
    out: Vec<u8>,
    pos: usize,
    e8_size: u32,

    r: [u32; 3],

    main_elements: u16,
    header_read: bool,

    main_lens: Vec<u8>,
    length_lens: Vec<u8>,
    aligned_lens: [u8; 8],

    main_tree: Option<Tree>,
    length_tree: Option<Tree>,
    align_tree: Option<Tree>,

    block_remaining: usize,
    block_type: Option<BlockType>,
}

#[derive(Debug)]
pub enum BlockType {
    Verbatim = 1,
    Aligned = 2,
    Uncompressed = 3,
}

impl LZXDecoder {
    pub fn new(window_bits: u8, out_len: usize) -> Result<Self, EnactError> {
        if !(15..=21).contains(&window_bits) {
            return Err(EnactError::InvalidWindowSize { size: window_bits });
        }

        let posn_slots = match window_bits {
            20 => 42,
            21 => 50,
            other => (other as u16) << 1,
        };

        let main_elements = 256 + 8 * posn_slots;
        let out = vec![0xDC; out_len];

        let main_lens = vec![0; (main_elements + 64) as usize];
        let length_lens = vec![0; 249 + 64];

        trace!(
            "created LZX decoder with main element size {}, out size {}, main length size {}",
            main_elements,
            out.len(),
            main_lens.len()
        );

        Ok(Self {
            out,
            pos: 0,
            e8_size: 0,
            r: [1; 3],
            main_elements,
            header_read: false,
            main_lens,
            length_lens,
            aligned_lens: [0; 8],
            block_remaining: 0,
            block_type: None,
            main_tree: None,
            length_tree: None,
            align_tree: None,
        })
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn decode_frame(
        &mut self,
        br: &mut BitReader<'_>,
        out_len: usize,
    ) -> Result<(), EnactError> {
        if !self.header_read {
            if br.bits(1)? != 0 {
                let hi = br.bits(16)?;
                let lo = br.bits(16)?;
                self.e8_size = (hi << 16) | lo;
            }
            self.header_read = true;
        }

        let frame_end = self.pos + out_len;

        while self.pos < frame_end {
            if self.block_remaining == 0 {
                let block_type = match br.bits(3)? {
                    1 => BlockType::Verbatim,
                    2 => BlockType::Aligned,
                    3 => BlockType::Uncompressed,
                    other => return Err(EnactError::BadLZXBlockType { t: other }),
                };

                let hi = br.bits(8)?;
                let lo = br.bits(16)?;
                let block_length = ((hi << 16) | lo) as usize;

                trace!("block {:?}, length {}", block_type, block_length);

                if matches!(block_type, BlockType::Uncompressed) {
                    return Err(EnactError::NotImplemented);
                }

                if matches!(block_type, BlockType::Aligned) {
                    for i in 0..8 {
                        self.aligned_lens[i] = br.bits(3)? as u8;
                    }
                    self.align_tree = Some(Tree::from_lengths(&self.aligned_lens));
                }

                Tree::read_lengths(br, &mut self.main_lens, 0, 256)?;
                Tree::read_lengths(br, &mut self.main_lens, 256, self.main_elements as usize)?;
                Tree::read_lengths(br, &mut self.length_lens, 0, 249)?;

                self.main_tree = Some(Tree::from_lengths(
                    &self.main_lens[..self.main_elements as usize],
                ));
                self.length_tree = Some(Tree::from_lengths(&self.length_lens[..249]));

                self.block_remaining = block_length;
                self.block_type = Some(block_type);
            }

            let take = min(self.block_remaining, frame_end - self.pos);
            let start = self.pos;

            let main = self.main_tree.take().ok_or(EnactError::NoTree)?;
            let lengths = self.length_tree.take().ok_or(EnactError::NoTree)?;
            let aligned = self.align_tree.take();
            let use_aligned = matches!(self.block_type, Some(BlockType::Aligned));

            let result = self.decode_run(
                br,
                take,
                &main,
                &lengths,
                if use_aligned { aligned.as_ref() } else { None },
            );

            self.main_tree = Some(main);
            self.length_tree = Some(lengths);
            self.align_tree = aligned;
            result?;

            self.block_remaining = self.block_remaining.saturating_sub(self.pos - start);
        }

        br.align();

        Ok(())
    }

    fn decode_run(
        &mut self,
        br: &mut BitReader<'_>,
        take: usize,
        main: &Tree,
        lengths: &Tree,
        aligned: Option<&Tree>,
    ) -> Result<(), EnactError> {
        let start = self.pos;

        while self.pos - start < take {
            let sym = main.decode(br)?;

            if sym < 256 {
                self.out[self.pos] = sym as u8;
                self.pos += 1;

                continue;
            }

            let length_header = ((sym - 256) & 7) as usize;
            let position_slot = ((sym - 256) >> 3) as usize;

            let length = if length_header == 7 {
                lengths.decode(br)? as usize + 9
            } else {
                length_header + 2
            };

            let offset = match position_slot {
                0 => self.r[0] as usize,
                1 => {
                    self.r.swap(0, 1);
                    self.r[0] as usize
                }

                2 => {
                    self.r.swap(0, 2);
                    self.r[0] as usize
                }

                _ => {
                    let extra = EXTRA_BITS[position_slot] as u32;
                    let verbatim = match aligned {
                        Some(tree) if extra >= 3 => {
                            (br.bits(extra - 3)? << 3) | tree.decode(br)? as u32
                        }
                        _ => br.bits(extra)?,
                    };

                    let offset = POSITION_BASE[position_slot] as usize + verbatim as usize - 2;

                    self.r[2] = self.r[1];
                    self.r[1] = self.r[0];
                    self.r[0] = offset as u32;

                    offset
                }
            };

            if offset > self.pos || self.pos + length > self.out.len() {
                return Err(EnactError::CorruptedStream);
            }

            for _ in 0..length {
                self.out[self.pos] = self.out[self.pos - offset];
                self.pos += 1;
            }
        }

        Ok(())
    }

    pub fn into_output(self) -> Vec<u8> {
        self.out
    }
}

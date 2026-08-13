use crate::{error::EnactError, reader::Reader};
use std::fmt::Debug;

#[derive(Debug)]
pub struct XNBTexture2D {
    pub surface_format: SurfaceFormat,
    pub width: u32,
    pub height: u32,
    pub mip_count: u32,
    pub mips: Vec<XNBImageMIP>,
}

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

#[repr(i32)]
#[derive(Debug)]
pub enum SurfaceFormat {
    Color,
    BGR565,
    BGRA5551,
    BGRA4444,
    DXT1,
    DXT3,
    DXT5,
    NormalizedByte2,
    NormalizedByte4,
    RGBA1010102,
    RG32,
    RGBA64,
    Alpha8,
    Single,
    Vector2,
    Vector4,
    HalfSingle,
    HalfVector2,
    HalfVector4,
    HDRBlendable,
}

impl TryFrom<i32> for SurfaceFormat {
    type Error = EnactError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Color,
            1 => Self::BGR565,
            2 => Self::BGRA5551,
            3 => Self::BGRA4444,
            4 => Self::DXT1,
            5 => Self::DXT3,
            6 => Self::DXT5,
            7 => Self::NormalizedByte2,
            8 => Self::NormalizedByte4,
            9 => Self::RGBA1010102,
            10 => Self::RG32,
            11 => Self::RGBA64,
            12 => Self::Alpha8,
            13 => Self::Single,
            14 => Self::Vector2,
            15 => Self::Vector4,
            16 => Self::HalfSingle,
            17 => Self::HalfVector2,
            18 => Self::HalfVector4,
            19 => Self::HDRBlendable,

            _ => return Err(EnactError::UnknownSurfaceFormat(value)),
        })
    }
}

impl XNBImageMIP {
    pub fn from_reader(reader: &mut Reader) -> Result<Self, EnactError> {
        let size = reader.u32_le()?;
        let data = reader.bytes(size as usize)?;

        Ok(Self {
            size,
            data: data.to_vec(),
        })
    }
}

impl XNBTexture2D {
    pub fn from_reader(mut reader: &mut Reader) -> Result<Self, EnactError> {
        let surface_format = SurfaceFormat::try_from(reader.i32_le()?)?;
        let width = reader.u32_le()?;
        let height = reader.u32_le()?;
        let mip_count = reader.u32_le()?;

        let mut mips = Vec::with_capacity(mip_count as usize);
        for _ in 0..mip_count {
            let mip = XNBImageMIP::from_reader(&mut reader)?;

            mips.push(mip);
        }

        Ok(Self {
            surface_format,
            width,
            height,
            mip_count,
            mips,
        })
    }
}

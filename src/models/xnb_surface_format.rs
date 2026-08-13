use crate::error::EnactError;

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

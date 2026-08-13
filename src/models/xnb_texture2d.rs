use crate::{
    error::EnactError,
    models::{xnb_image_mip::XNBImageMIP, xnb_surface_format::SurfaceFormat},
    readers::byte_reader::ByteReader,
};

use std::fmt::Debug;

#[derive(Debug)]
pub struct XNBTexture2D {
    pub surface_format: SurfaceFormat,
    pub width: u32,
    pub height: u32,
    pub mip_count: u32,
    pub mips: Vec<XNBImageMIP>,
}

impl XNBTexture2D {
    pub fn from_reader(mut reader: &mut ByteReader) -> Result<Self, EnactError> {
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

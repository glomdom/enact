use enact_parser::{
    error::EnactError,
    models::{xnb_effect::XNBEffect, xnb_file::XNBFile, xnb_texture2d::XNBTexture2D},
    readers::{byte_reader::ByteReader, type_reader::TypeReader},
};

use std::{collections::HashMap, fs, str::FromStr};
use tracing::{info, trace};
use tracing_subscriber::{EnvFilter, fmt};

fn main() -> anyhow::Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("enact_parser=trace,enact_extractor=trace")),
        )
        .with_writer(std::io::stderr)
        .init();

    let buf = fs::read("Acc_Back_1.xnb")?;
    let mut reader = ByteReader::new(&buf);

    let file = XNBFile::from_reader(&mut reader)?;

    let mut out_reader = ByteReader::new(&file.data);

    let type_readers_count = out_reader.encoded_int_7bit()? as usize;
    let mut type_readers = HashMap::new();

    for reader_num in 0..type_readers_count {
        let reader_full_name = out_reader.string()?;
        let reader_name = reader_full_name.split_once(',').unwrap().0.to_owned();
        let _version = out_reader.u32_le()?;

        type_readers.insert(reader_num, TypeReader::from_str(&reader_name)?);
    }

    let _shared_resources_count = out_reader.encoded_int_7bit()? as usize; // todo

    let primary_asset_id = out_reader.encoded_int_7bit()? as usize;
    let asset_reader = type_readers
        .get(&(primary_asset_id - 1))
        .ok_or(EnactError::NoTypeReader)?;

    trace!(?asset_reader);

    match asset_reader {
        TypeReader::Effect => {
            let effect = XNBEffect::from_reader(&mut out_reader)?;

            trace!(?effect);
        }

        TypeReader::Texture2D => {
            let image = XNBTexture2D::from_reader(&mut out_reader)?;

            trace!(?image);
        }
    }

    info!("{:#?}", file);

    Ok(())
}

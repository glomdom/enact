use std::fs;

use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

use crate::{models::xnb_file::XNBFile, readers::byte_reader::ByteReader};

mod error;
mod lzx;
mod models;
mod readers;

fn main() -> anyhow::Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("enact=trace")),
        )
        .with_writer(std::io::stderr)
        .init();

    let buf = fs::read("Acc_Back_1.xnb")?;
    let mut reader = ByteReader::new(&buf);

    let file = XNBFile::from_reader(&mut reader)?;

    info!("{:#?}", file);

    Ok(())
}

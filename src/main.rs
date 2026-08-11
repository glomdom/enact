use std::fs;

use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

use crate::{models::xnb_file::XNBFile, reader::Reader};

mod bit_reader;
mod error;
mod lzx;
mod models;
mod reader;

fn main() -> anyhow::Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("enact=trace")),
        )
        .with_writer(std::io::stderr)
        .init();

    let buf = fs::read("Acc_Back_1.xnb")?;
    let mut reader = Reader::new(&buf);

    let file = XNBFile::from_reader(&mut reader)?;

    info!("{:#?}", file);

    Ok(())
}

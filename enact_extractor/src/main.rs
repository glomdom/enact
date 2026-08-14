use std::fs;

use enact_parser::{models::xnb_file::XNBFile, readers::byte_reader::ByteReader};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

fn main() -> anyhow::Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("enact_parser=trace,enact_extractor=trace")),
        )
        .with_writer(std::io::stderr)
        .init();

    let buf = fs::read("TileShader.xnb")?;
    let mut reader = ByteReader::new(&buf);

    let file = XNBFile::from_reader(&mut reader)?;

    info!("{:#?}", file);

    Ok(())
}

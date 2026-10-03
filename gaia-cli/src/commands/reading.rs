//! Store a local reading. It is not a live feed.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ReadingArgs {
    pub lat: f64,
    pub lon: f64,
    pub celsius: f64,
    pub source: String,
}

pub async fn run(args: ReadingArgs) -> Result<()> {
    let reading = gaia_kernel::reading::ingest(args.lat, args.lon, args.celsius, &args.source)
        .map_err(anyhow::Error::msg)?;
    println!(
        "lat={} lon={} celsius={} source={} live={}",
        reading.lat, reading.lon, reading.celsius, reading.source, reading.live
    );
    Ok(())
}

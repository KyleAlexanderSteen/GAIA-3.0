use anyhow::Result;
use clap::Args;
use gaia_skills::{catalog, Band};

#[derive(Args)]
pub struct BandsArgs {}

pub async fn run(_args: BandsArgs) -> Result<()> {
    for row in catalog() {
        let band = match row.band {
            Band::Normal => "normal",
            Band::Super => "super",
            Band::Meta => "meta",
        };
        println!(
            "domain={} band={} shelf={} grants={}",
            row.domain, band, row.shelf, row.grants
        );
    }
    println!("meta describes the other bands and grants nothing");
    Ok(())
}

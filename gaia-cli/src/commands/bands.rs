use anyhow::Result;
use clap::Args;
use gaia_skills::{ascendence_self_claim, catalog, magic_is_meta_band, Band};

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
    println!(
        "meta describes knowledge, skills, and powers and grants nothing; magic_is_meta={} ascendence_self_claim={}",
        magic_is_meta_band(),
        ascendence_self_claim()
    );
    Ok(())
}

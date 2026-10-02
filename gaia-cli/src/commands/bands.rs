use anyhow::Result;
use clap::Args;
use gaia_skills::{
    ascendence_eligible, ascendence_self_claim, catalog, challenge_is_grant, declare_is_holding,
    entanglement_is_both, knowing_is_having, magic_is_meta_band, Band, Witness,
};

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
    let no_witness = Witness { outside_system: false, registered_failure: false, report_only: true };
    println!(
        "magic_is_meta={} knowing_is_having={} declare_is_holding={} challenge_is_grant={} entanglement_is_both={} ascendence_self_claim={} ascendence_eligible={}",
        magic_is_meta_band(),
        knowing_is_having(),
        declare_is_holding(),
        challenge_is_grant(),
        entanglement_is_both(),
        ascendence_self_claim(),
        ascendence_eligible(no_witness) == gaia_skills::Eligibility::EligibleForReview
    );
    Ok(())
}

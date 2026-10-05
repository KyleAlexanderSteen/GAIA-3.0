//! Call the pluralism gate and print what it returns.

use anyhow::{anyhow, Result};
use clap::Args;
use gaia_skills::pluralism::collapse;

#[derive(Args)]
pub struct LossArgs {
    pub kept: String,
    pub not_taken: String,
}

pub async fn run(args: LossArgs) -> Result<()> {
    match collapse(&args.kept, &args.not_taken) {
        Ok(loss) => {
            println!("kept={} not_taken={}", loss.kept, loss.not_taken);
            Ok(())
        }
        Err(reason) => Err(anyhow!("{reason}")),
    }
}

#[cfg(test)]
mod tests {
    use gaia_skills::pluralism::collapse;

    #[test]
    fn two_named_goods_are_refused_with_the_catalog_error() {
        let result = collapse("truth", "mercy");
        assert_eq!(
            result,
            Err("loss stays loss; catalog cannot close it"),
            "named goods must be refused without pretending the loss was resolved"
        );
    }

    #[test]
    fn identical_goods_are_rejected_as_non_collision() {
        assert_eq!(
            collapse("truth", "truth"),
            Err("one good is not a collision"),
            "identical goods must not be classified as a loss collision"
        );
    }
}

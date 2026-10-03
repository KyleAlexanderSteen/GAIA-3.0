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
    fn two_named_goods_reach_the_gate() {
        let result = collapse("truth", "mercy");
        assert!(result.is_err());
    }
}

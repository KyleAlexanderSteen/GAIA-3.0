//! Determine whether a named effect can be built.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct DetermineArgs {
    /// Effect name. Empty prints the known set.
    pub name: Option<String>,
}

pub async fn run(args: DetermineArgs) -> Result<()> {
    let effects = gaia_kernel::determine::known();
    let chosen: Vec<_> = match &args.name {
        Some(name) => effects.into_iter().filter(|effect| effect.name == name).collect(),
        None => effects,
    };
    if chosen.is_empty() {
        println!("known=0");
        return Ok(());
    }
    for effect in chosen {
        println!(
            "name={} pass={} reason={}",
            effect.name,
            gaia_kernel::determine::passes(&effect),
            gaia_kernel::determine::reason(&effect)
        );
    }
    Ok(())
}

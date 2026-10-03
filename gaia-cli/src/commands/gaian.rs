//! Make a companion. Unbounded is refused.

use anyhow::Result;
use gaia_kernel::gaian::ModelProvider;
use clap::Args;

#[derive(Args)]
pub struct GaianArgs {
    pub id: String,
    #[arg(long, default_value = "plain")]
    pub tone: String,
    #[arg(long, default_value = "observe")]
    pub autonomy: String,
}

pub async fn run(args: GaianArgs) -> Result<()> {
    let gaian = gaia_kernel::gaian::new(&args.id, &args.tone, &args.autonomy).map_err(anyhow::Error::msg)?;
    println!("id={} tone={} autonomy={}", gaian.id, gaian.tone, gaian.autonomy);
    match gaia_kernel::gaian::MissingLocal.complete("hello") {
        Ok(text) => println!("model={text}"),
        Err(reason) => println!("model={reason}"),
    }
    Ok(())
}

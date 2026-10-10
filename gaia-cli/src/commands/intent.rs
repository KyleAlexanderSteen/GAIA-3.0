use anyhow::{anyhow, Result};
use clap::Args;
use gaia_dispatch::dispatch_intent;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Args)]
pub struct IntentArgs {
    /// The intent text to submit
    pub text: String,

    /// Stream output as SSE chunks as it arrives
    #[arg(long, default_value_t = false)]
    pub stream: bool,

    /// Gateway base URL
    #[arg(long)]
    pub gateway: Option<String>,
}

pub async fn run(args: IntentArgs) -> Result<()> {
    if args.stream {
        return Err(super::not_implemented("intent --stream", "#1299"));
    }
    if args.gateway.is_some() {
        return Err(super::not_implemented("intent --gateway transport", "#1299"));
    }

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let id = format!("local-{millis}");
    let receipt = dispatch_intent(&id, &args.text);
    match receipt.status {
        "recorded" => {
            crate::ledger::append_recorded(&receipt.id, &receipt.detail)?;
            println!("recorded id={} detail={}", receipt.id, receipt.detail);
            Ok(())
        }
        other => Err(anyhow!("{other} {}", receipt.detail)),
    }
}

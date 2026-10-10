use anyhow::{anyhow, Context, Result};
use clap::Args;
use gaia_dispatch::dispatch_intent;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
struct GatewayIntentRequest<'a> {
    text: &'a str,
    profile: Option<&'a str>,
}

#[derive(Deserialize)]
struct GatewayIntentResponse {
    id: String,
    status: String,
    detail: String,
}

pub async fn run(args: IntentArgs) -> Result<()> {
    if args.stream {
        return Err(super::not_implemented("intent --stream", "#1299"));
    }

    if let Some(gateway) = args.gateway.as_deref() {
        let url = format!("{}/intent", gateway.trim_end_matches('/'));
        let response = reqwest::Client::new()
            .post(&url)
            .json(&GatewayIntentRequest {
                text: &args.text,
                profile: None,
            })
            .send()
            .await
            .with_context(|| format!("gateway intent request failed at {url}"))?;
        let status_code = response.status();
        let receipt: GatewayIntentResponse = response
            .json()
            .await
            .with_context(|| format!("gateway returned an invalid intent response at {url}"))?;

        if !status_code.is_success() || receipt.status != "recorded" {
            return Err(anyhow!(
                "gateway intent failed (HTTP {status_code}, status={}): {}",
                receipt.status,
                receipt.detail
            ));
        }

        println!("recorded id={} detail={}", receipt.id, receipt.detail);
        return Ok(());
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

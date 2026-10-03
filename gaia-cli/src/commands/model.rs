//! Local model call. No server is configured, so the call misses loud.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ModelArgs {
    /// Base URL of a local model server. Absent is a miss.
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn run(args: ModelArgs) -> Result<()> {
    Err(anyhow::anyhow!("{}", miss(args.base_url.as_deref())))
}

pub fn miss(base_url: Option<&str>) -> &'static str {
    match base_url {
        Some(url) if !url.is_empty() => "local model is not called yet; nothing was done",
        _ => "no local model server is set; nothing was done",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_server_is_a_loud_miss() {
        assert!(miss(None).contains("nothing was done"));
        assert!(miss(Some("")).contains("nothing was done"));
    }
}

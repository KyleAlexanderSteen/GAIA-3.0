use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct AuditArgs {
    /// Tail the audit log (follow mode)
    #[arg(long)]
    pub follow: bool,

    /// Maximum number of entries to show
    #[arg(long, default_value_t = 50)]
    pub limit: usize,
}

pub async fn run(args: AuditArgs) -> Result<()> {
    // TODO: GET /audit?limit=N  or WS /audit/stream when --follow
    let _ = args;
    Err(super::not_implemented("audit", "#1297"))
}

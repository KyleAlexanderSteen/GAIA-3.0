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
    if args.follow {
        return Err(super::not_implemented("audit --follow", "#1297"));
    }
    let rows = crate::ledger::read_recent(args.limit)?;
    if rows.is_empty() {
        println!("no local receipts");
        return Ok(());
    }
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

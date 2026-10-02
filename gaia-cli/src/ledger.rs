//! CLI view of the kernel receipt file.
use anyhow::Result;
use std::path::PathBuf;

pub fn ledger_path() -> PathBuf {
    gaia_kernel::receipts::ledger_path()
}

pub fn append_recorded(id: &str, detail: &str) -> Result<()> {
    gaia_kernel::receipts::append_recorded(id, detail).map_err(anyhow::Error::msg)
}

pub fn read_recent(limit: usize) -> Result<Vec<String>> {
    gaia_kernel::receipts::read_recent(limit).map_err(anyhow::Error::msg)
}

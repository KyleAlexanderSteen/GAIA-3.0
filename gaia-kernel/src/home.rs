//! File boundary. Writes stay under GAIA_HOME.

use std::path::{Path, PathBuf};

pub fn reject_outside(home: &Path, candidate: &Path) -> Result<PathBuf, &'static str> {
    let home = home.canonicalize().unwrap_or_else(|_| home.to_path_buf());
    if candidate.starts_with(&home) {
        Ok(candidate.to_path_buf())
    } else {
        Err("path is outside GAIA_HOME; nothing was done")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_path_is_rejected() {
        let home = Path::new("/tmp/gaia-home");
        assert!(reject_outside(home, Path::new("/etc/passwd")).is_err());
        assert!(reject_outside(home, Path::new("/tmp/gaia-home/profile")).is_ok());
    }
}

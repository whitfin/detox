//! Basic cleaner module for Cargo projects.
use super::Cleaner;
use std::io;

/// Cleaner implementation for Cargo projects.
pub struct CargoCleaner;
impl Cleaner for CargoCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Cargo"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &["Cargo.toml"]
    }

    /// cleaner the provided directory based on a Cargo structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "cargo", &["clean"])?;
        super::del(dir, "target")
    }
}

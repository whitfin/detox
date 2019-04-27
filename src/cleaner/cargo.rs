//! Basic cleaner module for Cargo projects.
use super::Cleaner;
use std::io;
use std::process::{Command, Stdio};

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
        Command::new("cargo")
            .arg("clean")
            .current_dir(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        super::purge(dir, "target")
    }
}

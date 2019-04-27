//! Basic cleansing module for Cargo projects.
use super::Cleanser;
use std::io;
use std::process::{Command, Stdio};

/// Cleanser implementation for Cargo projects.
pub struct CargoCleanser;
impl Cleanser for CargoCleanser {
    /// Returns the name of this cleanser.
    fn name(&self) -> &str {
        "Cargo"
    }

    /// Returns the triggers associated with this cleanser.
    fn triggers(&self) -> &[&str] {
        &["Cargo.toml"]
    }

    /// Cleanses the provided directory based on a Cargo structure.
    fn cleanse(&self, dir: &str) -> io::Result<()> {
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

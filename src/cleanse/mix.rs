//! Basic cleansing module for Mix projects.
use super::Cleanser;
use std::io;
use std::process::{Command, Stdio};

/// Cleanser implementation for Mix projects.
pub struct MixCleanser;
impl Cleanser for MixCleanser {
    /// Returns the name of this cleanser.
    fn name(&self) -> &str {
        "Mix"
    }

    /// Returns the triggers associated with this cleanser.
    fn triggers(&self) -> &[&str] {
        &["mix.exs"]
    }

    /// Cleanses the provided directory based on a Git structure.
    fn cleanse(&self, dir: &str) -> io::Result<()> {
        Command::new("mix")
            .arg("clean")
            .current_dir(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        super::purge(dir, "build")?;
        super::purge(dir, "deps")?;
        super::purge(dir, "doc")
    }
}

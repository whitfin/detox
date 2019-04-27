//! Basic cleaner module for Mix projects.
use super::Cleaner;
use std::io;
use std::process::{Command, Stdio};

/// Cleaner implementation for Mix projects.
pub struct MixCleaner;
impl Cleaner for MixCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Mix"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &["mix.exs"]
    }

    /// Cleans the provided directory based on a Git structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
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

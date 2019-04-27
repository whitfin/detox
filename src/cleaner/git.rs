//! Basic cleaner module for Git projects.
use super::Cleaner;
use std::io;
use std::process::{Command, Stdio};

/// Cleaner implementation for Git projects.
pub struct GitCleaner;
impl Cleaner for GitCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Git"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &[".git"]
    }

    /// Cleans the provided directory based on a Git structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
        Command::new("git")
            .arg("reflog")
            .arg("expire")
            .arg("--all")
            .arg("--expire=now")
            .current_dir(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        Command::new("git")
            .arg("gc")
            .arg("-q")
            .arg("--prune=now")
            .arg("--aggressive")
            .current_dir(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        Ok(())
    }
}

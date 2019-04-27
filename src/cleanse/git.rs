//! Basic cleansing module for Git projects.
use super::Cleanser;
use std::io;
use std::process::{Command, Stdio};

/// Cleanser implementation for Git projects.
pub struct GitCleanser;
impl Cleanser for GitCleanser {
    /// Returns the name of this cleanser.
    fn name(&self) -> &str {
        "Git"
    }

    /// Returns the triggers associated with this cleanser.
    fn triggers(&self) -> &[&str] {
        &[".git"]
    }

    /// Cleanses the provided directory based on a Git structure.
    fn cleanse(&self, dir: &str) -> io::Result<()> {
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

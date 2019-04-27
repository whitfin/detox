//! Basic cleaner module for Git projects.
use super::Cleaner;
use std::io;

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
        super::cmd(dir, "git", &["reflog", "expire", "--all", "--expire=now"])?;
        super::cmd(dir, "git", &["gc", "-q", "--prune=now", "--aggressive"])
    }
}

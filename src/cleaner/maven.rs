//! Basic cleaner module for Maven projects.
use super::Cleaner;
use std::io;

/// Cleaner implementation for Maven projects.
pub struct MavenCleaner;
impl Cleaner for MavenCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Maven"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &["pom.xml"]
    }

    /// Cleans the provided directory based on a Git structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "mvn", &["clean"])?;
        super::del(dir, "target")
    }
}

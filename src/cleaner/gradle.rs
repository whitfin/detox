//! Basic cleaner module for Gradle projects.
use super::Cleaner;
use std::io;

/// Cleaner implementation for Gradle projects.
pub struct GradleCleaner;
impl Cleaner for GradleCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Gradle"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &["build.gradle"]
    }

    /// Cleans the provided directory based on a Git structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "gradle", &["clean"])?;
        super::del(dir, "build")?;
        super::del(dir, "out")
    }
}

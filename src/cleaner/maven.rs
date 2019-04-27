//! Basic cleaner module for Maven projects.
use super::Cleaner;
use std::io;
use std::process::{Command, Stdio};

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
        Command::new("mvn")
            .arg("clean")
            .current_dir(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        super::purge(dir, "target")
    }
}

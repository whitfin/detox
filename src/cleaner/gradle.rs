//! Basic cleaner module for Gradle projects.
use super::Cleaner;
use std::io;
use std::process::{Command, Stdio};

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
        Command::new("gradle")
            .arg("clean")
            .current_dir(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        super::purge(dir, "build")?;
        super::purge(dir, "out")
    }
}

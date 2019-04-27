//! Basic cleansing module for Gradle projects.
use super::Cleanser;
use std::io;
use std::process::{Command, Stdio};

/// Cleanser implementation for Gradle projects.
pub struct GradleCleanser;
impl Cleanser for GradleCleanser {
    /// Returns the name of this cleanser.
    fn name(&self) -> &str {
        "Gradle"
    }

    /// Returns the triggers associated with this cleanser.
    fn triggers(&self) -> &[&str] {
        &["build.gradle"]
    }

    /// Cleanses the provided directory based on a Git structure.
    fn cleanse(&self, dir: &str) -> io::Result<()> {
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

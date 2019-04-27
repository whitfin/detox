//! Basic cleansing module for Maven projects.
use super::Cleanser;
use std::io;
use std::process::{Command, Stdio};

/// Cleanser implementation for Maven projects.
pub struct MavenCleanser;
impl Cleanser for MavenCleanser {
    /// Returns the name of this cleanser.
    fn name(&self) -> &str {
        "Maven"
    }

    /// Returns the triggers associated with this cleanser.
    fn triggers(&self) -> &[&str] {
        &["pom.xml"]
    }

    /// Cleanses the provided directory based on a Git structure.
    fn cleanse(&self, dir: &str) -> io::Result<()> {
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

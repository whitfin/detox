//! Basic cleaner module for Mix projects.
use super::Cleaner;
use std::io;
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
        super::cmd(dir, "mix", &["clean"])?;
        super::del(dir, "build")?;
        super::del(dir, "deps")?;
        super::del(dir, "doc")
    }
}

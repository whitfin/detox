//! Basic cleaner module for Node.js projects.
use super::Cleaner;
use std::io;

/// Cleaner implementation for Node.js projects.
pub struct NodeCleaner;
impl Cleaner for NodeCleaner {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Node.js"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &["package.json"]
    }

    /// Cleans the provided directory based on a Git structure.
    fn clean(&self, dir: &str) -> io::Result<()> {
        super::purge(dir, "node_modules")
    }
}

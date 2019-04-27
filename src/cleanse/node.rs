//! Basic cleansing module for Node.js projects.
use super::Cleanser;
use std::io;

/// Cleanser implementation for Node.js projects.
pub struct NodeCleanser;
impl Cleanser for NodeCleanser {
    /// Returns the name of this cleanser.
    fn name(&self) -> &str {
        "Node.js"
    }

    /// Returns the triggers associated with this cleanser.
    fn triggers(&self) -> &[&str] {
        &["package.json"]
    }

    /// Cleanses the provided directory based on a Git structure.
    fn cleanse(&self, dir: &str) -> io::Result<()> {
        super::purge(dir, "node_modules")
    }
}

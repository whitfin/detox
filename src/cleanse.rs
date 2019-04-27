//! Cleansing traits and implementations.
mod cargo;
mod git;
mod gradle;
mod maven;
mod mix;
mod node;

pub use cargo::CargoCleanser;
pub use git::GitCleanser;
pub use gradle::GradleCleanser;
pub use maven::MavenCleanser;
pub use mix::MixCleanser;
pub use node::NodeCleanser;

use std::fs;
use std::io::{self, ErrorKind};

/// Trait to represent a cleansing structure.
pub trait Cleanser {
    /// Returns the name of the current cleanser.
    fn name(&self) -> &str;

    /// Cleanses a directory assumed to be a relevant directory.
    fn cleanse(&self, dir: &str) -> io::Result<()>;

    /// Returns a set of file names which identify a relevant directory.
    fn triggers(&self) -> &[&str];
}

/// Purges a location on disk, similar to `rm -rf`.
pub fn purge(parent: &str, child: &str) -> io::Result<()> {
    let path = format!("{}/{}", parent, child);

    // check for errors that we're ok with
    if let Err(err) = fs::remove_dir_all(path) {
        // if already gone, happy days are upon us
        if err.kind() == ErrorKind::NotFound {
            return Ok(());
        }

        // if there's a permission error, we don't care
        if err.kind() == ErrorKind::PermissionDenied {
            return Ok(());
        }

        // others, bad!
        return Err(err);
    }

    Ok(())
}

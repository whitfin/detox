//! Cleaning traits and implementations.
mod cargo;
mod git;
mod gradle;
mod maven;
mod mix;
mod node;

pub use cargo::CargoCleaner;
pub use git::GitCleaner;
pub use gradle::GradleCleaner;
pub use maven::MavenCleaner;
pub use mix::MixCleaner;
pub use node::NodeCleaner;

use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Stdio};

/// Trait to represent a cleaning structure.
pub trait Cleaner {
    /// Returns the name of the current cleaner.
    fn name(&self) -> &str;

    /// Cleans a directory assumed to be a relevant directory.
    fn clean(&self, dir: &str) -> io::Result<()>;

    /// Returns a set of file names which identify a relevant directory.
    fn triggers(&self) -> &[&str];
}

/// Executes a command in a directory using provided arguments.
pub fn cmd(dir: &str, cmd: &str, args: &[&str]) -> io::Result<()> {
    Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    Ok(())
}

/// Purges a location on disk, similar to `rm -rf`.
pub fn del(parent: &str, child: &str) -> io::Result<()> {
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

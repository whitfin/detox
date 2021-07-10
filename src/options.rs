//! Options parsing and configuration for command line interfaces.
use clap::{App, AppSettings, Arg};
use std::ffi::OsString;
use std::path::PathBuf;

use crate::cleaner::*;

/// Options struct to store configuration state.
pub struct Options {
    pub(crate) cleaners: Vec<Box<dyn Cleaner>>,
    pub(crate) locations: Vec<PathBuf>,
}

impl Options {
    /// Creates an `Options` struct from an iterable set of arguments.
    pub fn from<I, T>(args: I) -> Options
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // create a new parser for our args
        let parser = Options::create_parser();

        // parse out the arguments into matching opts
        let options = parser.get_matches_from(args);

        // create opts
        Options {
            cleaners: vec![
                Box::new(GitCleaner),
                Box::new(GradleCleaner),
                Box::new(MixCleaner),
                Box::new(NodeCleaner),
                Box::new(MavenCleaner),
                Box::new(CargoCleaner),
            ],
            locations: options
                .values_of("locations")
                .unwrap()
                .filter_map(|location| PathBuf::from(location).canonicalize().ok())
                .collect(),
        }
    }

    /// Creates a parser used to generate `Options`.
    fn create_parser<'a, 'b>() -> App<'a, 'b> {
        App::new("")
            // package metadata from cargo
            .name(env!("CARGO_PKG_NAME"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .version(env!("CARGO_PKG_VERSION"))
            // arguments and flag details
            .args(&[
                // inputs: +required +multiple
                Arg::with_name("locations")
                    .help("Locations to search for cleanup")
                    .multiple(true)
                    .required(true),
            ])
            // settings required for parsing
            .settings(&[
                AppSettings::ArgRequiredElseHelp,
                AppSettings::HidePossibleValuesInHelp,
                AppSettings::TrailingVarArg,
            ])
    }
}

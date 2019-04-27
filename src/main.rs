//! Cleansing of unnecessary files in development directories.
#![doc(html_root_url = "https://docs.rs/cleanse/1.0.0")]
mod cleanse;
mod options;

use fs_extra::dir;
use walkdir::WalkDir;

use crate::options::Options;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    // parse in our options from the command line args
    let options = Options::from(&mut env::args_os());

    // iterate each provided location
    for location in &options.locations {
        // grab the size of the location before we start
        let start = dir::get_size(location)?;

        // iterate all file entries that we come across in the recursive walk
        for entry in WalkDir::new(location).into_iter().filter_map(Result::ok) {
            // grab the full path
            let path = entry.path();

            // fetch the file name
            let segment = path
                .file_name()
                .unwrap()
                .to_str()
                .expect("a segment should exist");

            // walk through all cleansers
            for cleanser in &options.cleansers {
                // skip if the cleanser doesn't care
                if !cleanser.triggers().contains(&segment) {
                    continue;
                }

                // grab the dir
                let dir = path
                    .parent()
                    .unwrap()
                    .to_str()
                    .expect("dir should be a str");

                // cleanse the directory
                cleanser.cleanse(&dir)?;
            }
        }

        // fetch the size of the location when done
        let end = dir::get_size(location)?;

        // output the stats
        println!(
            "Reduced {} from {} to {} ({:.2}%)",
            location,
            start,
            end,
            ((start - end) as f64 / start as f64) * 100.0
        )
    }

    // done!
    Ok(())
}

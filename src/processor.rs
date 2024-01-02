use crate::config::{Opt, OPTS};
use std::path::{Path, PathBuf};
use std::process;
use std::{fs, io};

use crate::iterator::process_paths;

pub fn absolute_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .into_iter()
        .filter_map(|path| match fs::canonicalize(&path) {
            Ok(abs_path) => Some(abs_path),
            Err(_) => {
                eprintln!(
                    "Error: Path does not exist or cannot be accessed: {:?}",
                    path
                );
                process::exit(1);
            }
        })
        .collect()
}


// Reads a directory's contents and returns a Result containing a Vec<String> of paths or an error
fn readdirone(path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    fs::read_dir(path)?
        .map(|res| res.map(|e| e.path())) // Convert the DirEntry into a PathBuf
        .collect() // Collect all the PathBuf instances into a Vec<PathBuf>
}

pub fn process() {
    let abs_paths = absolute_paths(OPTS.paths.clone());

    if OPTS.list && !abs_paths.is_empty() {
        // Check if the first path in abs_paths is a directory
        let first_path = &abs_paths[0];
        if Path::new(first_path).is_dir() {
            // Read the directory's content and process it
            match readdirone(first_path) {
                // Pass a reference here
                Ok(dir_content) => process_paths(dir_content),
                Err(e) => eprintln!("Failed to read directory: {}", e),
            }
        }
    } else {
        process_paths(abs_paths);
    }
}
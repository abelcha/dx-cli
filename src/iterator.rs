use std::{
    fs,
    path::{Path, PathBuf},
};

use bytesize::ByteSize;
use dx_cli::my_module::get_folder_size;

use crate::config;

extern crate libc;
// use libc::c_char;

fn get_file_size_in_bytes(path: &PathBuf) -> Result<u64, String> {
    if path.is_file() {
        fs::metadata(path)
            .map_err(|e| e.to_string())
            .and_then(|metadata| Ok(metadata.len()))
    } else {
        Err("The given path is not a file.".to_string())
    }
}

fn calculate_size(path: &PathBuf) -> u64 {
    if path.is_file() {
        if let Ok(file_size) = get_file_size_in_bytes(&path) {
            return file_size;
        }
    }
    return get_folder_size(path);
}

fn pad_end(s: &str, target_length: usize, pad_char: char) -> String {
    if s.len() >= target_length {
        return s.to_string();
    }
    let padding = pad_char.to_string().repeat(target_length - s.len());
    format!("{}{}", s, padding)
}

fn pad_start(s: &str, target_length: usize, pad_char: char) -> String {
    if s.len() >= target_length {
        return s.to_string();
    }
    let padding = pad_char.to_string().repeat(target_length - s.len());
    format!("{}{}", padding, s)
}

fn format_size(size: u64) -> String {
    if (config::OPTS.bytes) {
        return size.to_string();
    }
    let size_formatted = ByteSize::b(size).to_string();
    let rtn = size_formatted.replace('B', "").replace(' ', "");
    return rtn;
}

fn format_path(abs_path: &PathBuf, index: usize) -> String {
    if (config::OPTS.list) {
        let path_arg = &config::OPTS.paths[0];
        let file_name = abs_path.file_name().unwrap();
        let joined = path_arg.join(file_name);
        return joined.to_string_lossy().into_owned();
    }
    let current_path_arg = &config::OPTS.paths[index];
    if (current_path_arg.is_relative()) {
        let file_name: &std::ffi::OsStr = abs_path.file_name().unwrap();
        if (current_path_arg.ends_with(file_name)) {
            return current_path_arg.to_string_lossy().into_owned();
        }
        let joined = current_path_arg.join(file_name);
        return joined.to_string_lossy().into_owned();
    }
    return abs_path.to_string_lossy().into_owned();
}

pub fn process_paths(paths: Vec<PathBuf>) {
    for (index, path) in paths.iter().enumerate() {
        let path_formatted = format_path(path, index);
        let size = calculate_size(&path);
        let size_formatted = format_size(size);
        println!(
            "{} {}",
            pad_end(&size_formatted, 8, ' '),
            path_formatted,
            // path_arg.display(),
        );
    }
    // for path in paths {
    //     let size = calculate_size(&path);
    //     let size_formatted = format_size(size);
    //     println!("{} {}", pad_end(&size_formatted, 8, ' '), path.display());
    // }
}

// pub fn process_paths(paths: Vec<PathBuf>) {
//   // Use `par_iter` for parallel iteration
//   paths.par_iter().for_each(|path| {
//       let size = calculate_size(path);
//       let size_formatted = ByteSize::b(size).to_string();
//       println!("{} {}", pad_end(&size_formatted, 10, ' '), path.display());
//   });
// }

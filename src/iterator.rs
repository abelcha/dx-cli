use std::{
    fs,
    path::{Path, PathBuf},
};

use bytesize::ByteSize;

use crate::config;

extern crate libc;
// use libc::c_char;
use std::ffi::CString;

extern "C" {
    fn executeAppleScript(script: *const libc::c_char) -> *const libc::c_char;
}

pub fn run_apple_script(script: &str) -> String {
    let c_script = CString::new(script).expect("CString::new failed");
    unsafe {
        let result = executeAppleScript(c_script.as_ptr());
        std::ffi::CStr::from_ptr(result)
            .to_string_lossy()
            .into_owned()
    }
}

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
    let script_template: &'static str = include_str!("./get-folder-size.template");
    let script = script_template.replace('%', &path.to_string_lossy());
    let result = run_apple_script(&script.as_str());
    result.parse::<u64>().unwrap_or(0)
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
    let path_arg = &config::OPTS.paths[0];
    if (config::OPTS.list) {
        let file_name = abs_path.file_name().unwrap();
        let joined = path_arg.join(file_name);
        return joined.to_string_lossy().into_owned();
    }
    return path_arg.to_string_lossy().into_owned();
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

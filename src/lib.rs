#![allow(
    unused_variables,
    unreachable_code,
    unused_imports,
    dead_code,
    unused_parens
)]

pub mod byte_buffer;
pub mod config;
pub mod ds_parser;
pub mod ds_result;

#[derive(Debug, PartialEq, Eq)]
pub enum DataSource {
    Aev,
    DStore,
    Live,
    Failure,
}

pub mod fffs {
    use color_print::{cprint, cprintln};
    use strum_macros::Display;
    use walkdir::WalkDir;

    use std::{borrow::Borrow, ffi::CString, path::PathBuf, time::Duration};

    use crate::{
        config::{self, Strategy},
        ds_parser::{self, get_ds_cache, ModType},
        ds_result::ResultData,
        DataSource,
    };
    #[link(name = "fffs")]
    extern "C" {
        fn executeAppleScript(script: *const libc::c_char) -> *const libc::c_char;
        fn getFinderFastFolderSize(apath: *const libc::c_char) -> libc::c_longlong;
    }

    pub fn run_apple_script(script: &str) -> String {
        // let _lock = APPLE_SCRIPT_EXECUTOR.lock().unwrap();
        let c_script = CString::new(script).expect("CString::new failed");
        unsafe {
            let result = executeAppleScript(c_script.as_ptr());
            std::ffi::CStr::from_ptr(result)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_finder_fast_folder_size(path: &PathBuf) -> Result<i64, String> {
        let path_str = path.as_os_str().to_str().unwrap();

        let c_path: CString = CString::new(path_str).expect("CString::new failed");
        let resp = unsafe { getFinderFastFolderSize(c_path.as_ptr()) };
        if resp < 0 {
            let err = format!("Error - get_finder_fast_folder_size : {} ", resp);
            return Err(err);
        }
        return Ok(resp);
    }

    fn get_recursive_folder_size(path: &PathBuf) -> Result<i64, String> {
        // return Ok(0);
        let total_size = WalkDir::new(path)
            .min_depth(1)
            .max_depth(10)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .fold(0, |acc, m| acc + m.len());
        return Ok(total_size as i64);
    }

    fn get_file_size_in_bytes(path: &PathBuf) -> Result<i64, String> {
        std::fs::metadata(path)
            .map_err(|e| e.to_string())
            .and_then(|metadata| Ok(metadata.len() as i64))
    }

    #[derive(Debug, Clone)]
    pub struct PathResult {
        pub path: PathBuf,
        pub size: i64,
        pub duration: Duration,
        pub strategy: Option<Strategy>,
        pub error_message: Option<String>,
    }

    impl PathResult {
        fn new_success(path: &PathBuf, size: i64, duration: Duration, strategy: Strategy) -> Self {
            PathResult {
                path: path.clone(),
                size,
                duration,
                strategy: Some(strategy),
                error_message: None,
            }
        }
        fn new_error(path: &PathBuf, error_message: String) -> Self {
            PathResult {
                path: path.clone(),
                size: 0,
                duration: Duration::ZERO,
                strategy: None,
                error_message: Some(error_message),
            }
        }
    }

    pub fn process_path(path: &PathBuf, strategies: Vec<Strategy>) -> PathResult {
        if (path.is_file()) {
            return get_file_size_in_bytes(path).map_or_else(
                |err| PathResult::new_error(path, err),
                |size| PathResult::new_success(path, size, Duration::ZERO, Strategy::Live),
            );
        }
        let strats = if strategies.len() > 0 {
            strategies.clone()
        } else {
            vec![Strategy::Aev, Strategy::Dstore, Strategy::Live]
        };
        // println!("strategy: {:?}", strats);

        let result = strats.iter().find_map(|strategy| {
            let now = std::time::Instant::now();
            match strategy {
                Strategy::Aev => get_finder_fast_folder_size(path)
                    .ok()
                    .map(|size| PathResult::new_success(path, size, now.elapsed(), Strategy::Aev)),
                Strategy::Dstore => ds_parser::get_file_prop(path, ModType::LogicalSize)
                    .ok()
                    .map(|size| {
                        PathResult::new_success(path, size, now.elapsed(), Strategy::Dstore)
                    }),
                Strategy::Live => get_recursive_folder_size(path)
                    .ok()
                    .map(|size| PathResult::new_success(path, size, now.elapsed(), Strategy::Live)), // Strategy::Osa => None,
            }
        });
        return result
            .unwrap_or_else(|| PathResult::new_error(path, "No strategy worked".to_string()));
    }

    pub fn get_fffs(path: &PathBuf) -> Result<i64, String> {
        let rr = process_path(path, vec![Strategy::Dstore, Strategy::Aev, Strategy::Live]);
        match rr.error_message {
            Some(err) => Err(err),
            None => Ok(rr.size),
        }
    }
}

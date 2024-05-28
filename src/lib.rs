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
        fn runOsaScript(script: *const libc::c_char) -> *const libc::c_char;
        fn getFinderFastFolderSize(apath: *const libc::c_char) -> libc::c_longlong;
    }

    pub fn run_osa(script: &str) -> String {
        // let _lock = APPLE_SCRIPT_EXECUTOR.lock().unwrap();
        let c_script = CString::new(script).expect("CString::new failed");
        unsafe {
            let result = runOsaScript(c_script.as_ptr());
            std::ffi::CStr::from_ptr(result)
                .to_string_lossy()
                .into_owned()
        }
    }
    pub fn get_osa_folder_size(path: &PathBuf) -> Result<i64, String> {
        let script_template: &'static str = include_str!("./get-folder-size.template");
        let script = script_template.replace('%', &path.to_string_lossy());
        let result = run_osa(&script.as_str());
        result.parse::<i64>().map_err(|_| result)
    }

    pub fn get_finder_fast_folder_size(path: &PathBuf) -> Result<i64, String> {
        let path_str = path.as_os_str().to_str().unwrap();

        let c_path: CString = CString::new(path_str).expect("CString::new failed");
        let resp = unsafe { getFinderFastFolderSize(c_path.as_ptr()) };
        if resp < 0 {
            let err = format!("Error - get_finder_fast_folder_size : {} ", resp);
            return Err(err);
        }
        Ok(resp)
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
        Ok(total_size as i64)
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
        start_time: std::time::Instant,
    }

    impl PathResult {
        fn new(path: &PathBuf) -> Self {
            PathResult {
                path: path.clone(),
                size: 0,
                duration: Duration::ZERO,
                strategy: None,
                error_message: None,
                start_time: std::time::Instant::now(),
            }
        }
        // This method updates the fields of an existing PathResult instance
        fn success(&self, size: i64, strategy: Strategy) -> Self {
            Self {
                size,
                duration: self.start_time.elapsed(),
                strategy: Some(strategy),
                ..self.clone()
            }
        }
        // This method updates the fields of an existing PathResult instance
        fn error(&self, error_message: String) -> Self {
            // self.duration = self.start_time.elapsed();
            Self {
                error_message: Some(error_message),
                ..self.clone()
            }
        }
    }

    pub fn process_path(path: &PathBuf, strategies: Vec<Strategy>) -> PathResult {
        let result = PathResult::new(path);
        if (path.is_file()) {
            return get_file_size_in_bytes(path).map_or_else(
                |err| result.error(err),
                |size| result.success(size, Strategy::Live),
            );
        }
        let strats = if strategies.len() > 0 {
            strategies.clone()
        } else {
            match std::env::var("DX_STRATEGY") {
                Ok(val) => match val.as_str() {
                    "aev" => vec![Strategy::Aev],
                    "dstore" => vec![Strategy::Dstore],
                    "live" => vec![Strategy::Live],
                    _ => vec![Strategy::Aev, Strategy::Dstore, Strategy::Live],
                },
                Err(_) => vec![Strategy::Aev, Strategy::Dstore, Strategy::Live],
            }
            // vec![Strategy::Aev, Strategy::Dstore, Strategy::Live]
        };
        // println!("strategy: {:?}", strats);

        let result = strats.iter().find_map(|strategy| {
            // let now = std::time::Instant::now();
            let pathres = PathResult::new(path);
            match strategy {
                Strategy::Aev => get_finder_fast_folder_size(path)
                    .ok()
                    .map(|size| pathres.success(size, Strategy::Aev)),
                Strategy::Dstore => ds_parser::get_file_prop(path, ModType::LogicalSize)
                    .ok()
                    .map(|size| pathres.success(size, Strategy::Dstore)),
                Strategy::Live => get_recursive_folder_size(path)
                    .ok()
                    .map(|size| pathres.success(size, Strategy::Live)),
                Strategy::Osa => get_osa_folder_size(path)
                    .ok()
                    .map(|size| pathres.success(size, Strategy::Osa)),
            }
        });
        result.unwrap_or_else(|| PathResult::new(path).error("No strategy worked".to_string()))
    }

    pub fn get_fffs(path: &PathBuf) -> Result<i64, String> {
        let rr = process_path(path, vec![Strategy::Dstore, Strategy::Aev, Strategy::Live]);
        match rr.error_message {
            Some(err) => Err(err),
            None => Ok(rr.size),
        }
    }
}

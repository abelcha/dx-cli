use bytesize::ByteSize;
use color_print::{cformat, cprintln};
use dx_cli::fffs::{get_finder_fast_folder_size, process_path, PathResult};
use dx_cli::DataSource;
use itertools::Itertools;
use std::{
    borrow::Borrow,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Command, CommandArgs},
    time::{Duration, Instant},
};

use regex::Regex;

use dx_cli::config::{self, Strategy};

extern crate libc;

pub fn pad_end(s: &str, target_length: usize, pad_char: char) -> String {
    if s.len() >= target_length {
        return s.to_string();
    }
    let padding = pad_char.to_string().repeat(target_length - s.len());
    format!("{}{}", s, padding)
}

pub fn pad_start(s: &str, target_length: usize, pad_char: char) -> String {
    if s.len() >= target_length {
        return s.to_string();
    }
    let padding = pad_char.to_string().repeat(target_length - s.len());
    format!("{}{}", padding, s)
}

fn format_size(size: i64) -> String {
    if (config::ArgOpts.bytes) {
        return size.to_string();
    }
    if (size < 0) {
        return "KO".to_string();
    }
    let size_formatted = ByteSize::b(size as u64).to_string();
    let mut rtn = size_formatted.replace('B', "").replace(' ', "");
    if rtn.len() >= 5 {
        let re = Regex::new(r"\.[0-9]+").unwrap();
        rtn = re.replace(&rtn, "").into_owned();
    }
    if rtn.parse::<f64>().is_ok() {
        if (rtn.len() <= 3) {
            rtn = format!("{}B", rtn);
        }
    }
    return rtn;
}

fn format_path(abs_path: &PathBuf, index: usize) -> String {
    if (config::ArgOpts.list) {
        let path_arg = &config::ArgOpts.paths[0];
        let file_name = abs_path.file_name().unwrap();
        let joined = path_arg.join(file_name);

        return joined.to_string_lossy().into_owned();
    }
    let current_path_arg = &config::ArgOpts.paths[index];
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

fn format_timing(duration: Duration) -> String {
    if (!config::ArgOpts.perf) {
        return format!("");
    }
    if duration.as_secs_f32() > 1.0 {
        return cformat!(" <red>{:.1}s</red>", duration.as_secs_f32());
    }
    if duration.as_secs_f32() > 0.1 {
        return cformat!(" <yellow>{:.3}μ</yellow>", duration.subsec_millis());
    }
    if duration.as_secs_f32() > 0.01 {
        return format!(" {:3}μ", duration.subsec_millis());
    }
    return format!("     ");
}

fn pretty_print(path_index: usize, path_result: &PathResult) {
    let PathResult {
        path,
        size,
        strategy,
        duration,
        error_message,
        ..
    } = path_result;

    let size_formatted = format_size(*size);
    let home_dir = std::env::var("HOME").unwrap();
    let path_formatted = format_path(path, path_index).replace(home_dir.as_str(), "~");
    let pad_size = pad_start(&size_formatted, 5, ' ');
    let perf_timing = format_timing(*duration);

    let strat_indicator = match (config::ArgOpts.verbose, strategy) {
        (false, _) => "".to_string(),
        (true, Some(strategy)) => strategy.to_colored_short_name(),
        (true, None) => format!(
            "<red>{}</>",
            error_message.clone().unwrap_or("err".to_string())
        ),
    };
    cprintln!(
        "{}{} <bold>{}</> {}",
        strat_indicator,
        perf_timing,
        pad_size,
        path_formatted
    );
}

pub fn process_paths(paths: Vec<PathBuf>) {
    let vecstrats = config::ArgOpts.strategy.to_vec();

    let mapped_results = paths
        .iter()
        .filter(|path| !config::ArgOpts.dironly || path.is_dir())
        .map(|path| {
            if path.is_file() {
                process_path(path, vec![Strategy::Live])
            } else {
                process_path(path, vecstrats.clone())
            }
        })
        .inspect(|path_result| {
            if (!config::ArgOpts.sort) {
                pretty_print(0, path_result);
            }
        })
        .collect::<Vec<PathResult>>();
    if (config::ArgOpts.sort) {
        mapped_results.iter()
            .enumerate()
            .sorted_by_key(|&(_, p)| p.size)
            .for_each(|(index, path_result)| pretty_print(index, path_result));
    }
}

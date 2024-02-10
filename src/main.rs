#![allow(
    unused_variables,
    unreachable_code,
    unused_imports,
    dead_code,
    unused_parens
)]
mod config;
mod iterator;
mod processor;
mod ds_parser;
mod ds_result;
mod byte_buffer;

// use config::Opt;
use lazy_static::lazy_static;
use structopt::StructOpt;

use std::{
    path::{Path, PathBuf},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    // let opt: Opt = Opt::from_args();
    processor::process();

    if (config::OPTS.verbose) {
        println!("time elapsed: {}", now.elapsed().as_secs_f64());
    }
}

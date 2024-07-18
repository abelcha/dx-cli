#![allow(
    unused_variables,
    unreachable_code,
    unused_imports,
    dead_code,
    unused_parens
)]
// pub mod config;
mod config;
mod iterator;
mod processor;


use color_print::cprintln;
// use config::Opt;
use lazy_static::lazy_static;
use structopt::StructOpt;

use std::{
    path::{Path, PathBuf},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    // let args_opts: Opt = Opt::from_args();
    // if (config::ArgOpts.strategy.len() == 0) {
    //     config::ArgOpts.strategy = vec![config::Strategy::Aev, config::Strategy::DStore, config::Strategy::Live];
    //     println!("No strategy specified");
    //     return;
    // }
    processor::process();
    // println!("{:?}", config::ArgOpts.strategy[0]);

    if (config::ArgOpts.verbose) {
        cprintln!("\n<bold>time elapsed: <yellow>{:.3}ms</yellow>\n", now.elapsed().as_secs_f32());
    }
}

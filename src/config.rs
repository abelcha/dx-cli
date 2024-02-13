#![allow(non_upper_case_globals)]
use color_print::cformat;
use lazy_static::lazy_static;
use std::path::PathBuf;
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};
use strum_macros::Display;

#[derive(StructOpt, Debug)]
#[structopt(name = "dx")]
pub struct Opt {
    /// List
    #[structopt(short, long)]
    pub list: bool,

    /// Bytes
    #[structopt(short, long)]
    pub bytes: bool,

    /// Sort by size
    #[structopt(long)]
    pub sort: bool,

    /// Traces
    #[structopt(long)]
    pub trace: bool,

    /// Performance
    #[structopt(short, long)]
    pub perf: bool,

    /// directory only
    #[structopt(long)]
    pub dironly: bool,

    #[structopt(
            long,
            short,
            possible_values = Strategy::VARIANTS,
            case_insensitive = true,
            max_values = 3,
            min_values = 1,
        )]
    pub strategy: Vec<Strategy>,


    #[structopt(short, long)]
    pub verbose: bool,

    /// Paths
    #[structopt(name = "PATH", parse(from_os_str), default_value = "./")]
    pub paths: Vec<PathBuf>,
}

#[derive(EnumString, VariantNames, Debug, Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Strategy {
    Aev,
    Dstore,
    Live,
    Osa,
}

impl Strategy {
    pub fn to_colored_short_name(&self) -> String {
        match self {
            Strategy::Aev => cformat!("<cyan>[aev]"),
            Strategy::Dstore => cformat!("<green>[dst]"),
            Strategy::Live => cformat!("<magenta>[liv]"),
            Strategy::Osa => cformat!("<yellow>[osa]"),
        }
    }
}

lazy_static! {
    #[derive(Debug, Clone, Copy)]
    pub static ref ArgOpts: Opt = Opt::from_args();
}

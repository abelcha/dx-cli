use std::path::PathBuf;
use lazy_static::lazy_static;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "dx")]
pub struct Opt {
    /// List
    #[structopt(short, long)]
    pub list: bool,

    /// Bytes
    #[structopt(short, long)]
    pub bytes: bool,

    // /// Human-readable
    // #[structopt(short, long)]
    // human_readable: bool,

    // /// Depth
    // #[structopt(short, long, default_value = "0")]
    // depth: i32,
    /// Multi-threaded
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub multi_threaded: bool,

    /// Verbose
    #[structopt(short, long, parse(try_from_str), default_value = "false")]
    pub verbose: bool,
    /// Paths
    #[structopt(name = "PATH", parse(from_os_str), default_value = "./")]

    pub paths: Vec<PathBuf>,
}

lazy_static! {
  pub static ref OPTS: Opt = Opt::from_args();
}
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// Path to the file to be read
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    /// Display response headers
    #[structopt(short, long)]
    pub verbose: bool,

    /// If set, output won't be colored
    #[structopt(long = "no-color")]
    pub no_color: bool,
}

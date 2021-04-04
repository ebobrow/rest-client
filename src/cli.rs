use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// Path to the file to be read
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
    /// If true, response headers will not be displayed
    #[structopt(short, long)]
    pub concise: bool,
}

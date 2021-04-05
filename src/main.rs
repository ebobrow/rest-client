use rest_cli::cli::Cli;
use rest_cli::run;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    run(args);
}

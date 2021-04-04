use rest_client::cli::Cli;
use rest_client::run;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    run(args);
}

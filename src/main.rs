use clap_verbosity_flag::Verbosity;
use env_logger;
use human_panic::setup_panic;
use structopt::StructOpt;

mod error;

use error::Result;

fn main() -> Result<()> {
    setup_panic!();
    env_logger::init();
    let args = Cli::from_args();

    println!("{:?}", args);

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: Verbosity,
}

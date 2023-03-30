use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use env_logger;
use human_panic::setup_panic;

use feedback_manager::error::Result;

use feedback_manager::feedback::FeedbackItem;
use feedback_manager::io::{read_feedback, save_feedback};

fn main() -> Result<()> {
    setup_panic!();
    let args = Cli::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut feedback_data = if args.data_file.exists() {
        let input_file = File::open(&args.data_file)?;
        read_feedback(&input_file)?
    } else {
        Vec::new()
    };

    match args.command {
        Command::Feedback { person } => {
            log::info!("Registering feedback for {}", person);
            let feedback = FeedbackItem { person };
            feedback_data.push(feedback);
            {
                let output_file = File::create(&args.data_file)?;
                save_feedback(&output_file, &feedback_data)?;
            };
        }
        Command::List => {
            for feedback in feedback_data {
                println!("{}", feedback.person);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,

    /// The file to store the feedback in. Defaults to `~/.feedback.toml`.
    #[arg(short, long, default_value = "~/.feedback.json")]
    data_file: PathBuf,

    /// The command to run.
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    /// Register feedback for someone.
    Feedback {
        /// The person to register feedback for.
        #[arg(value_name = "PERSON", required = true)]
        person: String,
    },

    List,
}

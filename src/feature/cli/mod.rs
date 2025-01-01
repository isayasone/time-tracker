use clap::{Parser, Subcommand};
use error_stack::Result;

use crate::feature::tracker::flatfile::{FlatFileTracker, StartupStatus};

#[derive(Debug, thiserror::Error)]
#[error("a cli error occured")]
pub struct CliError;
#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    // start tracking time
    Start,
    Stop,
    Report,
}
#[derive(Debug, Clone, Copy, Parser)]
#[command(version, about,arg_required_else_help(true),long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub fn run() -> Result<(), CliError> {
    let args = Cli::parse();
    let tracker = FlatFileTracker::new("db.json", "lockfile.json");
    match args.command {
        Command::Start => {
            let state = tracker.start().unwrap();
            if state == StartupStatus::Running {
                println!(" tracking already started");
            } else {
                println!("Starting tracking time...");
            }
        }
        Command::Stop => {
            println!("Stopping tracking time...");
            tracker.stop().unwrap();
        }
        Command::Report => {
            println!("Generating report...");
            // Add logic to generate report
        }
    }

    Ok(())
}

use clap::{Parser, Subcommand};
use error_stack::Result;

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

    match args.command {
        Command::Start => {
            println!("Starting tracking time...");
            // Add logic to start tracking time
        }
        Command::Stop => {
            println!("Stopping tracking time...");
            // Add logic to stop tracking time
        }
        Command::Report => {
            println!("Generating report...");
            // Add logic to generate report
        }
    }

    Ok(())
}

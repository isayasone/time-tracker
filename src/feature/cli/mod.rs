use std::{path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use error_stack::{Result, ResultExt};

use crate::{
    error::Suggestion,
    feature::{
        report_fmt::{DurationFormat, HMSFormatter},
        tracker::{
            reporter::{ReportTimespan, Reporter},
            Tracker,
        },
    },
};

use super::tracker::{flatfile::FlatFileTracker, StartupStatus};

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
#[derive(Debug, Clone, Parser)]
#[command(version, about, arg_required_else_help(true))]
struct Cli {
    #[arg(short = 'd', long)]
    pub db_dir: Option<PathBuf>,
    #[arg(short = 'l', long)]
    pub lockfile: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

pub fn run() -> Result<(), CliError> {
    let args = Cli::parse();
    let db_dir = flatfile_db_dir(&args)?;
    let lockfile = lockfile_path(&args)?;
    let mut tracker = FlatFileTracker::new(db_dir, lockfile);
    match args.command {
        Command::Start => {
            let state = tracker.start().unwrap();
            if state == StartupStatus::Running {
                println!("Tracking already started");
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
            let twenty_four_hours = {
                const TWENTY_FOUR_HOURS: u64 = 60 * 60 * 24;
                Duration::from_secs(TWENTY_FOUR_HOURS)
            };
            let duration = tracker
                .total_duration(ReportTimespan::Last(twenty_four_hours))
                .change_context(CliError)
                .attach_printable("failed to calculate total track duration")?;
            let formatter = HMSFormatter::default();
            print!("{}", formatter.format(duration));
        }
    }

    Ok(())
}

fn lockfile_path(args: &Cli) -> Result<PathBuf, CliError> {
    match &args.lockfile {
        Some(lockfile) => Ok(lockfile.clone()),
        None => {
            let mut lockfile = dirs::cache_dir()
                .ok_or(CliError)
                .attach_printable("failed to discover cache directory")
                .attach(Suggestion("use the -l flag to specify a lockfile path"))?;

            lockfile.push("track");

            std::fs::create_dir_all(&lockfile)
                .change_context(CliError)
                .attach_printable("failed  to created 'track' locfile dirctory ")?;
            lockfile.push("lockfile.json");
            Ok(lockfile)
        }
    }
}

fn flatfile_db_dir(args: &Cli) -> Result<PathBuf, CliError> {
    match &args.db_dir {
        Some(db_dir) => Ok(db_dir.clone()),
        None => {
            let mut db_dirs = dirs::data_dir()
                .ok_or(CliError)
                .attach_printable("failed to discover data directory")
                .attach(Suggestion("use the -d flag to specify a database path"))?;

            db_dirs.push("track");

            std::fs::create_dir_all(&db_dirs)
                .change_context(CliError)
                .attach_printable("failed  to created 'track' db dirctory ")?;
            db_dirs.push("records.json");
            Ok(db_dirs)
        }
    }
}

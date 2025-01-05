use super::{
    reporter::Reporter, EndTime, StartTime, StartupStatus, TimeRecord, Tracker, TrackerError,
};
use error_stack::{Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Result as FmtResult,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    vec,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LockfileData {
    startTime: StartTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct FlatfileDatabase {
    records: Vec<TimeRecord>,
}

impl FlatfileDatabase {
    pub fn push(&mut self, value: TimeRecord) {
        self.records.push(value);
    }
}

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

pub struct FlatFileTracker {
    db: PathBuf,
    lockfile: PathBuf,
}

impl FlatFileTracker {
    pub fn new<D, L>(db: D, lockfile: L) -> Self
    where
        D: Into<PathBuf>,
        L: Into<PathBuf>,
    {
        let db = db.into();
        let lockfile = lockfile.into();
        Self { db, lockfile }
    }

    fn start_impl(&self) -> Result<StartupStatus, FlatFileTrackerError> {
        if self.is_running() {
            return Ok(StartupStatus::Running);
        }
        let lockfile_data = {
            let start_time = StartTime::now();
            let data = LockfileData {
                startTime: start_time,
            };
            serde_json::to_string(&data)
                .change_context(FlatFileTrackerError)
                .attach_printable("failed to serialize lockfile data")?
        };
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to create new lockfile")?
            .write_all(lockfile_data.as_bytes())
            .change_context(FlatFileTrackerError)
            .attach_printable("failed to write lockfile data")?;
        return Ok(StartupStatus::Started);
    }

    fn stop_impl(&self) -> Result<(), FlatFileTrackerError> {
        let start = read_lockfile(&self.lockfile)?;

        let end = EndTime::now();

        let record = TimeRecord { start, end };
        let mut db = load_database(&self.db)?;
        db.push(record);
        save_database(&self.db, db)?;

        std::fs::remove_file(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to remove lockfile")?;
        Ok(())
    }
}

impl Reporter for FlatFileTracker {}
impl Tracker for FlatFileTracker {
    fn start(&mut self) -> Result<StartupStatus, TrackerError> {
        self.start_impl().change_context(TrackerError)
    }

    fn is_running(&self) -> bool {
        self.lockfile.exists()
    }

    fn stop(&mut self) -> Result<(), TrackerError> {
        self.stop_impl().change_context(TrackerError)
    }

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError> {
        let db = load_database(&self.db).change_context(TrackerError)?;

        Ok(db.records.into_iter())
    }
}

fn save_database<P>(path: P, db: FlatfileDatabase) -> Result<(), FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    let db = serde_json::to_string(&db)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to serialize database data")?;
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Fixed to overwrite the file correctly
        .open(path.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("unable to open database")?
        .write_all(db.as_bytes())
        .change_context(FlatFileTrackerError)
        .attach_printable("unable to write database")?;
    Ok(())
}

fn load_database<P>(db: P) -> Result<FlatfileDatabase, FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    if !db.as_ref().exists() {
        // Return an empty database if the file does not exist
        return Ok(FlatfileDatabase::default());
    }

    let mut db_buf = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .open(db.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("unable to open database")?;

    file.read_to_string(&mut db_buf)
        .change_context(FlatFileTrackerError)
        .attach_printable("unable to read database")?;

    if db_buf.is_empty() {
        return Ok(FlatfileDatabase::default());
    }

    Ok(serde_json::from_str(&db_buf)
        .change_context(FlatFileTrackerError)
        .attach_printable("unable to deserialize database data")?)
}

fn read_lockfile<P>(lockfile: P) -> Result<StartTime, FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    let file = OpenOptions::new()
        .read(true)
        .open(lockfile.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("unable to open lockfile")?;
    let data: LockfileData = serde_json::from_reader(file)
        .change_context(FlatFileTrackerError)
        .attach_printable("unable to deserialize lockfile data")?;
    Ok(data.startTime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::{fixture::PathChild, TempDir};

    fn tracking_paths() -> (TempDir, PathBuf, PathBuf) {
        let temp = TempDir::new().unwrap();
        let lockfile = temp.child("lockfile").path().to_path_buf();
        let db = temp.child("db.json").path().to_path_buf();
        (temp, lockfile, db)
    }

    #[test]
    fn is_running_true_after_starting_tracker() {
        let (_tempdir, lockfile, db) = tracking_paths();
        let mut tracker = FlatFileTracker::new(db, lockfile);

        tracker.start().unwrap();

        assert!(tracker.is_running());
    }

    #[test]
    fn is_running_false_after_stopping_tracker() {
        let (_tempdir, lockfile, db) = tracking_paths();
        let mut tracker = FlatFileTracker::new(db, lockfile);

        tracker.start().unwrap();
        tracker.stop().unwrap();

        assert!(!tracker.is_running());
    }

    #[test]
    fn time_record_created_when_tracking_stops() {
        let (_tempdir, lockfile, db) = tracking_paths();
        let mut tracker = FlatFileTracker::new(db, lockfile);

        tracker.start().unwrap();
        tracker.stop().unwrap();

        assert!(tracker.records().unwrap().count() > 0);
    }

    #[test]
    fn intial_start_returns_already_started_state() {
        let (_tempdir, lockfile, db) = tracking_paths();
        let mut tracker = FlatFileTracker::new(db, lockfile);

        //when the tracker is started again
        let started = tracker.start().unwrap();

        //Then  already running state is returned

        assert_eq!(started, StartupStatus::Started);
    }

    #[test]
    fn multiple_start_returns_already_running_state() {
        let (_tempdir, lockfile, db) = tracking_paths();
        let mut tracker = FlatFileTracker::new(db, lockfile);

        tracker.start().unwrap();
        //when the tracker is started again
        let started = tracker.start().unwrap();

        //Then  astarted state is returned

        assert_eq!(started, StartupStatus::Running);
    }
}

use std::{fs::OpenOptions, path::PathBuf};

use error_stack::{Result, ResultExt};

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

pub struct FlatFileTracker {
    db: PathBuf,
    lockfile: PathBuf,
}

impl FlatFileTracker {
    fn new<D, L>(db: D, lockfile: L) -> Self
    where
        D: Into<PathBuf>,
        L: Into<PathBuf>,
    {
        let db = db.into();
        let lockfile = lockfile.into();
        Self { db, lockfile }
    }

    fn start(&self) -> Result<(), FlatFileTrackerError> {
        // if !self.lockfile.exists() {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to create new lockfile")?;
        // }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.lockfile.exists()
    }

    fn stop(&self) -> Result<(), FlatFileTrackerError> {
        std::fs::remove_file(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to remove lockfile")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::{fixture::PathChild, TempDir};

    use super::*;

    fn tracking_paths() -> (TempDir, PathBuf, PathBuf) {
        let temp = TempDir::new().unwrap();
        let lockfile = temp.child("lockfile").path().to_path_buf();
        let db = temp.child("db.json").path().to_path_buf();
        (temp, lockfile, db)
    }

    #[test]
    fn is_running_false_after_starting_tracker() {
        let (_tempdir, lockfile, db) = tracking_paths();
        //Given  a default tracker
        let tracker = FlatFileTracker::new(db, lockfile);

        //when the tracker is started

        tracker.stop().unwrap();

        //Then the tracker  is  running

        assert!(tracker.is_running());
    }

    #[test]
    fn is_running_false_after_stopping_tracker() {
        let (_tempdir, lockfile, db) = tracking_paths();

        //Given  a default tracker
        let tracker = FlatFileTracker::new(db, lockfile);

        //when the tracker is started

        tracker.start().unwrap();

        tracker.stop().unwrap();
        //Then the tracker  is   no longer running

        assert!(!tracker.is_running());
    }
    #[test]
    fn time_record_created_when_tracking_stops() {
        let (_tempdir, lockfile, db) = tracking_paths();

        //Given  a default tracker
        let tracker = FlatFileTracker::new(db, lockfile);

        //when the tracker is started

        tracker.start().unwrap();

        tracker.stop().unwrap();
        //Then the tracker  is   no longer running

        assert!(!tracker.is_running());
    }
}

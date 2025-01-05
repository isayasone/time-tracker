use chrono::Utc;
use error_stack::Result;
use error_stack::ResultExt;
use std::{str, time::Duration};

use super::Tracker;

#[derive(Debug, Clone, Copy)]
enum ReportTimespan {
    Last(Duration),
}

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
struct ReporterError;

pub trait Reporter: Tracker {
     fn total_duration(&self, timespan: ReportTimespan) -> Result<Duration, ReporterError> {
        match timespan {
            ReportTimespan::Last(timespan) => {
                let target = (Utc::now() - timespan).timestamp_millis();

                let total_ms = self
                    .records()
                    .change_context(ReporterError)
                    .attach("failed to query records")?
                    .filter_map(|rec| {
                        if rec.start.timestamp_millis() >= target {
                            let ms = rec.end.timestamp_millis() - rec.start.timestamp_millis();
                            Some(ms)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>();

                Ok(Duration::from_millis(total_ms as u64))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use assert_cmd::assert;
    use ttlib::FakeTracker;

    use super::*;

    #[test]
    fn calculate_correct_duration_when_there_are_no_records() {
        let mut tracker = FakeTracker::default();

        let duration = tracker
            .total_duration(ReportTimespan::Last(Duration::from_secs(1)))
            .unwrap();

        assert_eq!(duration, Duration::from_millis(0));
    }

    #[test]
    fn calculate_correct_duration_when_there_are_two_records() {
        let mut tracker = FakeTracker::default();

        tracker.start().unwrap();
        std::thread::sleep(Duration::from_millis(10));
        tracker.stop().unwrap();

        tracker.start().unwrap();
        std::thread::sleep(Duration::from_millis(10));
        tracker.stop().unwrap();

        let duration = tracker
            .total_duration(ReportTimespan::Last(Duration::from_secs(1)))
            .unwrap();

        assert!(duration >= Duration::from_millis(20));
    }
}

#[cfg(test)]
mod ttlib {
    use crate::feature::tracker::{
        EndTime, StartTime, StartupStatus, TimeRecord, Tracker, TrackerError,
    };

    use super::*;
    #[derive(Debug, Default)]
    pub struct FakeTracker {
        tracking: Option<StartTime>,
        records: Vec<TimeRecord>,
    }
    impl Tracker for FakeTracker {
        fn start(&mut self) -> Result<StartupStatus, TrackerError> {
            if self.tracking.is_some() {
                return Ok(StartupStatus::Running);
            }
            self.tracking = Some(StartTime::now());
            Ok(StartupStatus::Started)
        }

        fn is_running(&self) -> bool {
            self.tracking.is_some()
        }

        fn stop(&mut self) -> Result<(), TrackerError> {
            let start_time = self.tracking.take().unwrap();
            let end_time = EndTime::now();
            let record = TimeRecord {
                start: start_time,
                end: end_time,
            };
            self.records.push(record);
            Ok(())
        }

        fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError> {
            Ok(self.records.iter().copied())
        }
    }

    impl Reporter for FakeTracker {}
}

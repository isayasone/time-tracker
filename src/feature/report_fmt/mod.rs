#[derive(Default, Debug)]
pub struct HMSFormatter;
trait DurationFormat {
    fn format(&self, duration: std::time::Duration) -> String;
}

impl DurationFormat for HMSFormatter {
    fn format(&self, duration: std::time::Duration) -> String {
        let seconds = duration.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use assert_cmd::assert;

    use super::*;

    #[test]
    fn formats_seconds() {
        let duration = Duration::from_secs(5);
        let formatter = HMSFormatter::default();
        let text = formatter.format(duration);
        assert_eq!(text, "00:00:05");
    }
}

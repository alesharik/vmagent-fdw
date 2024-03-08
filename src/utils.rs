use chrono::{DateTime, Datelike, Timelike, Utc};
use pgrx::Timestamp;

pub fn from_chrono(ts: DateTime<Utc>) -> Timestamp {
    Timestamp::new_unchecked(
        ts.year() as isize,
        ts.month() as u8,
        ts.day() as u8,
        ts.hour() as u8,
        ts.minute() as u8,
        ts.second() as f64,
    )
}

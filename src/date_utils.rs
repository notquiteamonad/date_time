use date_tuple::DateTuple;
use month_tuple::MonthTuple;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const UNIX_EPOCH_START_YEAR: u16 = 1970;

/// Takes a year as a u16 and returns whether it is a leap year.
pub fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
}

/// Gets the current date as a `DateTuple`
pub fn now_as_datetuple() -> DateTuple {
    let mut seconds = duration_since_unix_epoch().as_secs();
    let parts = extract_year_and_month_from_duration(seconds);
    seconds = parts.2;
    let days = seconds / 86400 + 1; //Days past plus current
    DateTuple::new(parts.0, parts.1, days as u8).unwrap()
}

/// Gets the current month as a `MonthTuple`
pub fn now_as_monthtuple() -> MonthTuple {
    let seconds = duration_since_unix_epoch().as_secs();
    let parts = extract_year_and_month_from_duration(seconds);
    MonthTuple::new(parts.0, parts.1).unwrap()
}

/// Takes a duration in seconds and removes the year and month parts from it, returning
/// a tuple of the year, month, and remaining seconds.
///
/// Month is returned zero-based
fn extract_year_and_month_from_duration(mut seconds: u64) -> (u16, u8, u64) {
    let years = seconds / 31557600;
    seconds -= years * 31557600;
    let months = seconds / 2629800;
    seconds -= months * 2629800;
    (years as u16 + UNIX_EPOCH_START_YEAR, months as u8, seconds)
}

/// Gets a duration using `std::time::SystemTime::now()` since the
/// unix epoch.
fn duration_since_unix_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::new(0, 0))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_leap_years() {
        let valid: [u16; 3] = [2000, 2012, 2016];
        let invalid: [u16; 3] = [2100, 2018, 2013];
        for v in valid.iter() {
            assert!(super::is_leap_year(*v));
        }
        for i in invalid.iter() {
            assert!(!super::is_leap_year(*i));
        }
    }
}

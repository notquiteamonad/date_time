use date_tuple::DateTuple;
use month_tuple::MonthTuple;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time_tuple::TimeTuple;

const UNIX_EPOCH_START_YEAR: u16 = 1970;

const SECONDS_IN_A_YEAR: u64 = 31557600;
const SECONDS_IN_A_MONTH: u64 = 2629800;
const SECONDS_IN_A_DAY: u64 = 86400;

/// Takes a year as a u16 and returns whether it is a leap year.
pub fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
}

/// Produces the integer representing the last date in the **ZERO-BASED**
/// month in year.
pub fn get_last_date_in_month(month: u8, year: u16) -> u8 {
    match month {
        1 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        _ => 30,
    }
}

/// Gets the current date as a `DateTuple`
pub fn now_as_datetuple() -> DateTuple {
    let mut seconds = duration_since_unix_epoch().as_secs();
    let parts = extract_year_and_month_from_duration(seconds);
    seconds = parts.2;
    let days = seconds / SECONDS_IN_A_DAY + 1; //Days past plus current
    DateTuple::new(parts.0, parts.1, days as u8).unwrap()
}

/// Gets the current month as a `MonthTuple`
pub fn now_as_monthtuple() -> MonthTuple {
    let seconds = duration_since_unix_epoch().as_secs();
    let parts = extract_year_and_month_from_duration(seconds);
    MonthTuple::new(parts.0, parts.1).unwrap()
}

/// Gets the current time of day from `std::time::SystemTime` as a TimeTuple
pub fn now_as_timetuple() -> TimeTuple {
    let seconds = duration_since_unix_epoch().as_secs();
    TimeTuple::from_seconds(seconds)
}

/// Takes a duration in seconds and removes the year and month parts from it, returning
/// a tuple of the year, month, and remaining seconds.
///
/// Month is returned zero-based
fn extract_year_and_month_from_duration(mut seconds: u64) -> (u16, u8, u64) {
    let years = seconds / SECONDS_IN_A_YEAR;
    seconds -= years * SECONDS_IN_A_YEAR;
    let months = seconds / SECONDS_IN_A_MONTH;
    seconds -= months * SECONDS_IN_A_MONTH;
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

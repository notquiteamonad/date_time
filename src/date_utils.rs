use date_tuple::DateTuple;
use month_tuple::MonthTuple;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time_tuple::TimeTuple;

const SECONDS_IN_A_DAY: u64 = 86400;

lazy_static! {
    static ref UNIX_EPOCH_DATETUPLE: DateTuple = DateTuple::new(1970, 1, 1).unwrap();
}

/// Takes a year as a u16 and returns whether it is a leap year.
pub fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
}

/// Produces the integer representing the last date in the **ZERO-BASED**
/// month in year.
pub fn get_last_date_in_month(month: u8, year: u16) -> u8 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 30,
    }
}

/// Gets the current date as a `DateTuple`
pub fn now_as_datetuple() -> DateTuple {
    let seconds = duration_since_unix_epoch().as_secs();
    DateTuple::from_days(extract_days_from_duration(seconds) + UNIX_EPOCH_DATETUPLE.to_days())
        .unwrap()
}

/// Gets the current month as a `MonthTuple`
pub fn now_as_monthtuple() -> MonthTuple {
    MonthTuple::from(now_as_datetuple())
}

/// Gets the current time of day from `std::time::SystemTime` as a TimeTuple
pub fn now_as_timetuple() -> TimeTuple {
    let seconds = duration_since_unix_epoch().as_secs();
    TimeTuple::from_seconds(seconds)
}

/// Takes a duration in seconds and calculates the number of days in it.
fn extract_days_from_duration(seconds: u64) -> u32 {
    (seconds / SECONDS_IN_A_DAY) as u32
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

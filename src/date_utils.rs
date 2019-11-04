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

/// Produces the integer representing the last date in the month in year.
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
    use super::*;

    #[test]
    fn test_leap_years() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2012));
        assert!(is_leap_year(2016));
        assert!(!is_leap_year(2100));
        assert!(!is_leap_year(2018));
        assert!(!is_leap_year(2013));
    }

    #[test]
    fn test_now_functions_do_not_panic() {
        now_as_datetuple();
        now_as_monthtuple();
        now_as_timetuple();
    }

    #[test]
    fn test_days_from_duration() {
        assert_eq!(0, extract_days_from_duration(0));
        assert_eq!(0, extract_days_from_duration(500));
        assert_eq!(26, extract_days_from_duration(2246500));
    }

    #[test]
    fn test_duration_since_epoch() {
        assert!(duration_since_unix_epoch().as_secs() > 0);
    }
}

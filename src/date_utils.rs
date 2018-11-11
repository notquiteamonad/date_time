use date_tuple::DateTuple;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const UNIX_EPOCH_START_YEAR: u16 = 1970;

/// Takes a year as a u16 and returns whether it is a leap year.
pub fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
}

/// Gets the current date as a `DateTuple`
pub fn now_as_datetuple() -> DateTuple {
    let mut seconds = duration_since_unix_epoch().as_secs();
    let years = seconds / 31557600;
    seconds -= years * 31557600;
    let months = seconds / 2629800;
    seconds -= months * 2629800;
    let days = seconds / 86400 + 1; //Days past plus current
    DateTuple::new(
        years as u16 + UNIX_EPOCH_START_YEAR,
        months as u8,
        days as u8,
    ).unwrap()
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

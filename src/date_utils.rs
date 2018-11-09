use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Takes a year as a u16 and returns whether it is a leap year.
pub fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
}

/// Gets a duration using `std::time::SystemTime::now()` since the
/// unix epoch.
pub fn duration_since_unix_epoch() -> Duration {
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

use date_utils;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

pub type Time = TimeTuple;

/// A wrapper for a particular time of day or a duration.
///
/// Precise to second-level.
///
/// **NOTE:** This cannot be 24 hours or greater - see `TimeTuple::new()` for more details.
///
/// The wrapping described in `TimeTuple::new()` also applies when adding and subtracting times.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct TimeTuple {
    h: u8,
    m: u8,
    s: u8,
}

impl TimeTuple {
    /// Produces a new TimeTuple.
    ///
    /// Times of 24 hours or greater and negative times
    /// will wrap around 24 hours to always produce a positive time.
    ///
    /// The value is calculated from total number of seconds so a time
    /// with a minute value of 90 would add an hour to the resulting tuple
    /// and set the minutes to 30, for example.
    pub fn new(h: i32, m: i32, s: i32) -> TimeTuple {
        let mut total_seconds = s + 60 * m + 3600 * h;
        while total_seconds < 0 {
            total_seconds += 86400;
        }
        TimeTuple::from_seconds(total_seconds as u64)
    }

    /// Same as `TimeTuple::new()` but takes the total number of seconds
    /// as its argument and calculates the hours, minutes, and seconds
    /// from that.
    pub fn from_seconds(mut total_seconds: u64) -> TimeTuple {
        while total_seconds >= 86400 {
            total_seconds -= 86400;
        }
        let h = total_seconds / 3600;
        total_seconds -= h * 3600;
        let m = total_seconds / 60;
        total_seconds -= m * 60;
        TimeTuple {
            h: h as u8,
            m: m as u8,
            s: total_seconds as u8,
        }
    }

    /// Returns a `TimeTuple` of the current time as `std::time::SystemTime` provides it.
    pub fn now() -> TimeTuple {
        date_utils::now_as_timetuple()
    }

    pub fn get_hours(&self) -> u8 {
        self.h
    }

    pub fn get_minutes(&self) -> u8 {
        self.m
    }

    pub fn get_seconds(&self) -> u8 {
        self.s
    }

    /// Produces a string such as 08:30 for 8 hours and 30 minutes.
    ///
    /// Ignores seconds.
    pub fn to_hhmm_string(&self) -> String {
        format!("{:02}:{:02}", self.h, self.m)
    }

    /// Gets the total number of seconds in the tuple.
    pub fn to_seconds(&self) -> u32 {
        3600 * self.h as u32 + 60 * self.m as u32 + self.s as u32
    }

    /// Adds a number of seconds to the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn add_seconds(&mut self, seconds: i32) {
        let new_seconds = self.s as i32 + seconds;
        *self = TimeTuple::new(self.h as i32, self.m as i32, new_seconds);
    }

    /// Subtracts a number of seconds from the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn subtract_seconds(&mut self, seconds: i32) {
        let new_seconds = self.s as i32 - seconds;
        *self = TimeTuple::new(self.h as i32, self.m as i32, new_seconds);
    }

    /// Adds a number of minutes to the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn add_minutes(&mut self, minutes: i32) {
        let new_minutes = self.m as i32 + minutes;
        *self = TimeTuple::new(self.h as i32, new_minutes, self.s as i32);
    }

    /// Subtracts a number of minutes from the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn subtract_minutes(&mut self, minutes: i32) {
        let new_minutes = self.m as i32 - minutes;
        *self = TimeTuple::new(self.h as i32, new_minutes, self.s as i32);
    }

    /// Adds a number of hours to the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn add_hours(&mut self, hours: i32) {
        let new_hours = self.h as i32 + hours;
        *self = TimeTuple::new(new_hours, self.m as i32, self.s as i32);
    }

    /// Subtracts a number of hours from the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn subtract_hours(&mut self, hours: i32) {
        let new_hours = self.h as i32 - hours;
        *self = TimeTuple::new(new_hours, self.m as i32, self.s as i32);
    }
}

impl fmt::Display for TimeTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.h, self.m, self.s)
    }
}

impl FromStr for TimeTuple {
    type Err = String;

    fn from_str(s: &str) -> Result<TimeTuple, Self::Err> {
        let valid_format = Regex::new(r"^\d{2}:\d{2}:\d{2}$").unwrap();
        if !valid_format.is_match(s) {
            Err(format!(
                "Invalid str formatting of TimeTuple: {}\nExpects a string formatted like 08:30:05",
                s
            ))
        } else {
            let mut parts = s.split(':');
            Ok(TimeTuple::new(
                i32::from_str(parts.next().unwrap()).unwrap(),
                i32::from_str(parts.next().unwrap()).unwrap(),
                i32::from_str(parts.next().unwrap()).unwrap(),
            ))
        }
    }
}

impl PartialOrd for TimeTuple {
    fn partial_cmp(&self, other: &TimeTuple) -> Option<Ordering> {
        self.to_seconds().partial_cmp(&other.to_seconds())
    }
}

impl Ord for TimeTuple {
    fn cmp(&self, other: &TimeTuple) -> Ordering {
        self.to_seconds().cmp(&other.to_seconds())
    }
}

impl Add for TimeTuple {
    type Output = TimeTuple;
    fn add(self, other: TimeTuple) -> TimeTuple {
        TimeTuple::new(
            (self.h + other.h) as i32,
            (self.m + other.m) as i32,
            (self.s + other.s) as i32,
        )
    }
}

impl AddAssign for TimeTuple {
    fn add_assign(&mut self, other: TimeTuple) {
        *self = TimeTuple::new(
            (self.h + other.h) as i32,
            (self.m + other.m) as i32,
            (self.s + other.s) as i32,
        );
    }
}

impl Sub for TimeTuple {
    type Output = TimeTuple;
    fn sub(self, other: TimeTuple) -> TimeTuple {
        TimeTuple::new(
            (self.h - other.h) as i32,
            (self.m - other.m) as i32,
            (self.s - other.s) as i32,
        )
    }
}

impl SubAssign for TimeTuple {
    fn sub_assign(&mut self, other: TimeTuple) {
        *self = TimeTuple::new(
            (self.h - other.h) as i32,
            (self.m - other.m) as i32,
            (self.s - other.s) as i32,
        );
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_no_seconds() {
        let tuple = super::TimeTuple::new(5, 30, 0);
        assert_eq!(5, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_no_overlap() {
        let tuple = super::TimeTuple::new(5, 30, 30);
        assert_eq!(5, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_second_overlap() {
        let tuple = super::TimeTuple::new(6, 30, 90);
        assert_eq!(6, tuple.h);
        assert_eq!(31, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_minute_overlap() {
        let tuple = super::TimeTuple::new(6, 90, 30);
        assert_eq!(7, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_hour_overlap() {
        let tuple = super::TimeTuple::new(25, 30, 30);
        assert_eq!(1, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_all_overlap() {
        let tuple = super::TimeTuple::new(25, 90, 90);
        assert_eq!(2, tuple.h);
        assert_eq!(31, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_minutes_to_hours_overlap() {
        let tuple = super::TimeTuple::new(18, 420, 0);
        assert_eq!(1, tuple.h);
        assert_eq!(0, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_negative_seconds() {
        let tuple = super::TimeTuple::new(6, 30, -60);
        assert_eq!(6, tuple.h);
        assert_eq!(29, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_all_negative_overlaps() {
        let tuple = super::TimeTuple::new(-3, -116, -301);
        assert_eq!(18, tuple.h);
        assert_eq!(58, tuple.m);
        assert_eq!(59, tuple.s);
    }

    #[test]
    fn test_to_string() {
        let tuple = super::TimeTuple::new(3, 0, 39);
        assert_eq!(String::from("03:00:39"), tuple.to_string())
    }

    #[test]
    fn test_to_hhmm_string() {
        let tuple = super::TimeTuple::new(3, 0, 39);
        assert_eq!(String::from("03:00"), tuple.to_hhmm_string())
    }

    #[test]
    fn test_operators() {
        let zeroes = super::TimeTuple::new(0, 0, 0);
        let ones = super::TimeTuple::new(1, 1, 1);
        let twos = super::TimeTuple::new(2, 2, 2);
        assert_eq!(twos, ones + ones);
        assert_eq!(zeroes, ones - ones);
        assert!(zeroes < ones);
        assert!(zeroes < twos);
        assert!(twos > ones);
        assert!(zeroes <= ones);
        assert!(ones <= ones);
    }

    #[test]
    fn test_to_seconds() {
        let tuple = super::TimeTuple::new(2, 30, 30);
        assert_eq!(9030, tuple.to_seconds())
    }

    #[test]
    fn test_from_seconds() {
        let tuple = super::TimeTuple::from_seconds(86400);
        assert_eq!(super::TimeTuple::new(0, 0, 0), tuple);
    }

    #[test]
    fn test_from_string() {
        let tuple = super::TimeTuple::new(5, 30, 4);
        assert_eq!(tuple, str::parse("05:30:04").unwrap());
    }

    #[test]
    fn test_manipulate_seconds() {
        let mut tuple = super::TimeTuple::new(10, 58, 59);
        tuple.add_seconds(3);
        assert_eq!(super::TimeTuple::new(10, 59, 2), tuple);
        tuple.subtract_seconds(1);
        tuple.subtract_seconds(2);
        assert_eq!(super::TimeTuple::new(10, 58, 59), tuple);
    }

    #[test]
    fn test_manipulate_minutes() {
        let mut tuple = super::TimeTuple::new(10, 58, 59);
        tuple.add_minutes(3);
        assert_eq!(super::TimeTuple::new(11, 1, 59), tuple);
        tuple.subtract_minutes(1);
        tuple.subtract_minutes(2);
        assert_eq!(super::TimeTuple::new(10, 58, 59), tuple);
    }

    #[test]
    fn test_manipulate_hours() {
        let mut tuple = super::TimeTuple::new(10, 58, 59);
        tuple.add_hours(3);
        assert_eq!(super::TimeTuple::new(13, 58, 59), tuple);
        tuple.subtract_hours(1);
        tuple.subtract_hours(2);
        assert_eq!(super::TimeTuple::new(10, 58, 59), tuple);
    }

}

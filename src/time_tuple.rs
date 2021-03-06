use crate::date_time_tuple::DateTimeTuple;
use date_utils;
use regex::Regex;
use std::cmp::Ordering;
use std::convert::From;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

pub type Time = TimeTuple;
pub type TimeOfDay = TimeTuple;

/// A wrapper for a particular time of day.
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

    pub fn get_hours(self) -> u8 {
        self.h
    }

    pub fn get_minutes(self) -> u8 {
        self.m
    }

    pub fn get_seconds(self) -> u8 {
        self.s
    }

    /// Produces a string such as 08:30 for 8 hours and 30 minutes.
    ///
    /// Ignores seconds.
    pub fn to_hhmm_string(self) -> String {
        format!("{:02}:{:02}", self.h, self.m)
    }

    /// Gets the total number of seconds in the tuple.
    pub fn to_seconds(self) -> u32 {
        3600 * u32::from(self.h) + 60 * u32::from(self.m) + u32::from(self.s)
    }

    /// Gets the total number of minutes in the tuple, ignoring seconds.
    pub fn to_minutes(self) -> u32 {
        60 * u32::from(self.h) + u32::from(self.m)
    }

    /// Adds a number of seconds to the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn add_seconds(&mut self, seconds: i32) {
        let new_seconds = i32::from(self.s) + seconds;
        *self = TimeTuple::new(i32::from(self.h), i32::from(self.m), new_seconds);
    }

    /// Subtracts a number of seconds from the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn subtract_seconds(&mut self, seconds: i32) {
        let new_seconds = i32::from(self.s) - seconds;
        *self = TimeTuple::new(i32::from(self.h), i32::from(self.m), new_seconds);
    }

    /// Adds a number of minutes to the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn add_minutes(&mut self, minutes: i32) {
        let new_minutes = i32::from(self.m) + minutes;
        *self = TimeTuple::new(i32::from(self.h), new_minutes, i32::from(self.s));
    }

    /// Subtracts a number of minutes from the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn subtract_minutes(&mut self, minutes: i32) {
        let new_minutes = i32::from(self.m) - minutes;
        *self = TimeTuple::new(i32::from(self.h), new_minutes, i32::from(self.s));
    }

    /// Adds a number of hours to the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn add_hours(&mut self, hours: i32) {
        let new_hours = i32::from(self.h) + hours;
        *self = TimeTuple::new(new_hours, i32::from(self.m), i32::from(self.s));
    }

    /// Subtracts a number of hours from the TimeTuple,
    /// wrapping the same way `TimeTuple::new()` does.
    pub fn subtract_hours(&mut self, hours: i32) {
        let new_hours = i32::from(self.h) - hours;
        *self = TimeTuple::new(new_hours, i32::from(self.m), i32::from(self.s));
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
        lazy_static! {
            static ref VALID_FORMAT: Regex = Regex::new(r"^\d{2}:\d{2}:\d{2}$").unwrap();
        }

        if !VALID_FORMAT.is_match(s) {
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

#[cfg_attr(tarpaulin, skip)]
impl Ord for TimeTuple {
    fn cmp(&self, other: &TimeTuple) -> Ordering {
        self.to_seconds().cmp(&other.to_seconds())
    }
}

impl Add for TimeTuple {
    type Output = TimeTuple;
    fn add(self, other: TimeTuple) -> TimeTuple {
        TimeTuple::new(
            i32::from(self.h + other.h),
            i32::from(self.m + other.m),
            i32::from(self.s + other.s),
        )
    }
}

impl AddAssign for TimeTuple {
    fn add_assign(&mut self, other: TimeTuple) {
        *self = TimeTuple::new(
            i32::from(self.h + other.h),
            i32::from(self.m + other.m),
            i32::from(self.s + other.s),
        );
    }
}

impl Sub for TimeTuple {
    type Output = TimeTuple;
    fn sub(self, other: TimeTuple) -> TimeTuple {
        TimeTuple::new(
            i32::from(self.h - other.h),
            i32::from(self.m - other.m),
            i32::from(self.s - other.s),
        )
    }
}

impl SubAssign for TimeTuple {
    fn sub_assign(&mut self, other: TimeTuple) {
        *self = TimeTuple::new(
            i32::from(self.h - other.h),
            i32::from(self.m - other.m),
            i32::from(self.s - other.s),
        );
    }
}

/// A wrapper for a duration.
///
/// Does not count in days, but can have hours >24 (up to `u32::MAX`)
///
/// Precise to second-level.
///
/// Cannot be negative.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Duration {
    h: u32,
    m: u8,
    s: u8,
}

impl Duration {
    /// Produces a new Duration.
    ///
    /// The value is calculated from total number of seconds so a duration
    /// with a minute value of 90 would add an hour to the resulting tuple
    /// and set the minutes to 30, for example.
    pub fn new(h: u32, m: u32, s: u32) -> Duration {
        let total_seconds: u64 = u64::from(s) + 60 * u64::from(m) + 3600 * u64::from(h);
        Duration::from_seconds(total_seconds)
    }

    /// Same as `Duration::new()` but takes the total number of seconds
    /// as its argument and calculates the hours, minutes, and seconds
    /// from that.
    pub fn from_seconds(mut total_seconds: u64) -> Duration {
        let h = total_seconds / 3600;
        total_seconds -= h * 3600;
        let m = total_seconds / 60;
        total_seconds -= m * 60;
        Duration {
            h: h as u32,
            m: m as u8,
            s: total_seconds as u8,
        }
    }

    /// Calculates the `Duration` between two `DateTimeTuple`s.
    pub fn between(dt1: DateTimeTuple, dt2: DateTimeTuple) -> Duration {
        if dt1 == dt2 {
            return Duration { h: 0, m: 0, s: 0 };
        }
        let smaller = if dt1 < dt2 { dt1 } else { dt2 };
        let greater = if dt1 < dt2 { dt2 } else { dt1 };
        let days_between = greater.get_date().to_days() - smaller.get_date().to_days();
        if days_between == 0 {
            Duration::from(greater.get_time()) - Duration::from(smaller.get_time())
        } else {
            let time_between = Duration::from(greater.get_time()) + Duration::new(24, 0, 0)
                - Duration::from(smaller.get_time());
            time_between + Duration::new(24 * (days_between - 1), 0, 0)
        }
    }

    pub fn get_hours(self) -> u32 {
        self.h
    }

    pub fn get_minutes(self) -> u8 {
        self.m
    }

    pub fn get_seconds(self) -> u8 {
        self.s
    }

    /// Produces a string such as 8:30 for 8 hours and 30 minutes.
    ///
    /// Hours field will expand as necessary; 150:30 is a possible result.
    ///
    /// Ignores seconds.
    pub fn to_hhmm_string(self) -> String {
        format!("{}:{:02}", self.h, self.m)
    }

    /// Produces a string such as 8:30 for 8 hours and 30 minutes.
    ///
    /// Hours field will expand as necessary; 150:30 is a possible result.
    ///
    /// Ignores seconds.
    #[deprecated(since = "2.1.0", note = "Replace with to_hhmm_string()")]
    pub fn to_hours_and_minutes_string(self) -> String {
        format!("{}:{:02}", self.h, self.m)
    }

    /// Gets the total number of seconds in the Duration.
    pub fn to_seconds(self) -> u64 {
        3600 * u64::from(self.h) + 60 * u64::from(self.m) + u64::from(self.s)
    }

    /// Gets the total number of minutes in the Duration, ignoring seconds.
    pub fn to_minutes(self) -> u32 {
        60 * self.h + u32::from(self.m)
    }

    /// Adds a number of seconds to the Duration,
    /// wrapping the same way `Duration::new()` does.
    pub fn add_seconds(&mut self, seconds: u32) {
        let new_seconds = u32::from(self.s) + seconds;
        *self = Duration::new(self.h, u32::from(self.m), new_seconds);
    }

    /// Subtracts a number of seconds from the Duration,
    /// wrapping the same way `Duration::new()` does.
    pub fn subtract_seconds(&mut self, seconds: u32) {
        *self = Duration::from_seconds(self.to_seconds() - u64::from(seconds));
    }

    /// Adds a number of minutes to the Duration,
    /// wrapping the same way `Duration::new()` does.
    pub fn add_minutes(&mut self, minutes: u32) {
        let new_minutes = u32::from(self.m) + minutes;
        *self = Duration::new(self.h, new_minutes, u32::from(self.s));
    }

    /// Subtracts a number of minutes from the Duration,
    /// wrapping the same way `Duration::new()` does.
    pub fn subtract_minutes(&mut self, minutes: u32) {
        *self = Duration::from_seconds(self.to_seconds() - u64::from(minutes) * 60);
    }

    /// Adds a number of hours to the Duration,
    /// wrapping the same way `Duration::new()` does.
    pub fn add_hours(&mut self, hours: u32) {
        let new_hours = self.h + hours;
        *self = Duration::new(new_hours, u32::from(self.m), u32::from(self.s));
    }

    /// Subtracts a number of hours from the Duration,
    /// wrapping the same way `Duration::new()` does.
    pub fn subtract_hours(&mut self, hours: u32) {
        let new_hours = self.h - hours;
        *self = Duration::new(new_hours, u32::from(self.m), u32::from(self.s));
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:02}:{:02}", self.h, self.m, self.s)
    }
}

impl FromStr for Duration {
    type Err = String;

    fn from_str(s: &str) -> Result<Duration, Self::Err> {
        lazy_static! {
            static ref VALID_FORMAT: Regex = Regex::new(r"^\d+:\d{2}:\d{2}$").unwrap();
        }
        if !VALID_FORMAT.is_match(s) {
            Err(format!(
                "Invalid str formatting of Duration: {}\nExpects a string formatted like 8:30:05",
                s
            ))
        } else {
            let mut parts = s.split(':');
            Ok(Duration::new(
                u32::from_str(parts.next().unwrap()).unwrap(),
                u32::from_str(parts.next().unwrap()).unwrap(),
                u32::from_str(parts.next().unwrap()).unwrap(),
            ))
        }
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Duration) -> Option<Ordering> {
        self.to_seconds().partial_cmp(&other.to_seconds())
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Ord for Duration {
    fn cmp(&self, other: &Duration) -> Ordering {
        self.to_seconds().cmp(&other.to_seconds())
    }
}

impl Add for Duration {
    type Output = Duration;
    fn add(self, other: Duration) -> Duration {
        Duration::new(
            self.h + other.h,
            u32::from(self.m + other.m),
            u32::from(self.s + other.s),
        )
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, other: Duration) {
        *self = Duration::new(
            self.h + other.h,
            u32::from(self.m + other.m),
            u32::from(self.s + other.s),
        );
    }
}

impl Sub for Duration {
    type Output = Duration;
    fn sub(self, other: Duration) -> Duration {
        Duration::new(
            self.h - other.h,
            u32::from(self.m - other.m),
            u32::from(self.s - other.s),
        )
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, other: Duration) {
        *self = Duration::new(
            self.h - other.h,
            u32::from(self.m - other.m),
            u32::from(self.s - other.s),
        );
    }
}

impl From<TimeTuple> for Duration {
    fn from(time: TimeTuple) -> Self {
        Duration::from_seconds(u64::from(time.to_seconds()))
    }
}

#[cfg(test)]
mod tests {

    use super::TimeTuple;

    #[test]
    fn test_no_seconds() {
        let tuple = TimeTuple::new(5, 30, 0);
        assert_eq!(5, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_no_overlap() {
        let tuple = TimeTuple::new(5, 30, 30);
        assert_eq!(5, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_second_overlap() {
        let tuple = TimeTuple::new(6, 30, 90);
        assert_eq!(6, tuple.h);
        assert_eq!(31, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_minute_overlap() {
        let tuple = TimeTuple::new(6, 90, 30);
        assert_eq!(7, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_hour_overlap() {
        let tuple = TimeTuple::new(25, 30, 30);
        assert_eq!(1, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_all_overlap() {
        let tuple = TimeTuple::new(25, 90, 90);
        assert_eq!(2, tuple.h);
        assert_eq!(31, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_minutes_to_hours_overlap() {
        let tuple = TimeTuple::new(18, 420, 0);
        assert_eq!(1, tuple.h);
        assert_eq!(0, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_negative_seconds() {
        let tuple = TimeTuple::new(6, 30, -60);
        assert_eq!(6, tuple.h);
        assert_eq!(29, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_all_negative_overlaps() {
        let tuple = TimeTuple::new(-3, -116, -301);
        assert_eq!(18, tuple.h);
        assert_eq!(58, tuple.m);
        assert_eq!(59, tuple.s);
    }
}

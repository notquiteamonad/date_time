use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct TimeTuple {
    h: i32,
    m: i32,
    s: i32,
}

impl TimeTuple {
    pub fn new(h: i32, m: i32, s: i32) -> TimeTuple {
        let mut total_seconds = s + 60 * m + 3600 * h;
        while total_seconds > 86400 {
            total_seconds -= 86400;
        }
        while total_seconds < 0 {
            total_seconds += 86400;
        }
        let h = total_seconds / 3600;
        total_seconds -= h * 3600;
        let m = total_seconds / 60;
        total_seconds -= m * 60;
        TimeTuple {
            h,
            m,
            s: total_seconds,
        }
    }

    pub fn get_hours(&self) -> u32 {
        self.h as u32
    }

    pub fn get_minutes(&self) -> u32 {
        self.m as u32
    }

    pub fn get_seconds(&self) -> u32 {
        self.s as u32
    }

    pub fn to_hhmm_string(&self) -> String {
        format!("{:02}:{:02}", self.h, self.m)
    }

    pub fn to_seconds(&self) -> i32 {
        3600 * self.h + 60 * self.m + self.s
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
            Err(format!("Invalid str formatting of TimeTuple: {}", s))
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
        TimeTuple::new(self.h + other.h, self.m + other.m, self.s + other.s)
    }
}

impl AddAssign for TimeTuple {
    fn add_assign(&mut self, other: TimeTuple) {
        *self = TimeTuple::new(self.h + other.h, self.m + other.m, self.s + other.s);
    }
}

impl Sub for TimeTuple {
    type Output = TimeTuple;
    fn sub(self, other: TimeTuple) -> TimeTuple {
        TimeTuple::new(self.h - other.h, self.m - other.m, self.s - other.s)
    }
}

impl SubAssign for TimeTuple {
    fn sub_assign(&mut self, other: TimeTuple) {
        *self = TimeTuple::new(self.h - other.h, self.m - other.m, self.s - other.s);
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
    fn test_from_string() {
        let tuple = super::TimeTuple::new(5, 30, 4);
        assert_eq!(tuple, str::parse("05:30:04").unwrap());
    }

}

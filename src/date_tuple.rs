use date_utils;
use month_tuple::MonthTuple;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

pub type Date = DateTuple;

/// Holds a specific date by year, month, and day.
///
/// Handles values from 01 Jan 0000 to 31 Dec 9999.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct DateTuple {
    y: u16,
    m: u8,
    d: u8,
}

impl DateTuple {
    /// Takes a year, month, and day and converts them into a DateTuple.
    ///
    /// Will not overlap - the date entered must be valid without further calculation.
    pub fn new(y: u16, m: u8, d: u8) -> Result<DateTuple, String> {
        if y > 9999 {
            return Err(format!(
                "Invalid year in DateTuple {:?}: year must be <= 9999.",
                DateTuple { y, m, d }
            ));
        }
        if m <= 11 {
            let max_date = match m {
                1 => {
                    if date_utils::is_leap_year(y as u16) {
                        29
                    } else {
                        28
                    }
                }
                0 => 31,
                2 => 31,
                4 => 31,
                6 => 31,
                7 => 31,
                9 => 31,
                11 => 31,
                _ => 30,
            };
            if d == 0 || d > max_date {
                return Err(format!(
                    "Invalid date in DateTuple: {:?}",
                    DateTuple { y, m, d }
                ));
            }
            Ok(DateTuple { y: y, m, d })
        } else {
            Err(format!(
                "Invalid month in DateTuple: {:?}\nMonth must be <= 11; Note that months are ZERO-BASED.",
                DateTuple { y, m, d }
            ))
        }
    }

    pub fn get_year(&self) -> u16 {
        self.y
    }

    pub fn get_month(&self) -> u8 {
        self.m
    }

    pub fn get_date(&self) -> u8 {
        self.d
    }

    /// Produces a readable date.
    ///
    /// ## Examples
    /// * 2 Oct 2018
    /// * 13 Jan 2019
    pub fn to_readable_string(&self) -> String {
        let month = MonthTuple::from(*self);
        format!("{} {}", self.d, month.to_readable_string())
    }
}

impl fmt::Display for DateTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}{:02}{:02}", self.y, self.m, self.d)
    }
}

impl FromStr for DateTuple {
    type Err = String;

    /// Expects a string formatted like 20181102.
    fn from_str(s: &str) -> Result<DateTuple, Self::Err> {
        let valid_format = Regex::new(r"^\d{8}$").unwrap();
        if !valid_format.is_match(s) {
            Err(format!("Invalid str formatting of DateTuple: {}\nExpects a string formatted like 20181102.", s))
        } else {
            let (s1, s2) = s.split_at(4);
            let (s2, s3) = s2.split_at(2);
            Ok(DateTuple::new(
                u16::from_str(s1).unwrap(),
                u8::from_str(s2).unwrap(),
                u8::from_str(s3).unwrap(),
            ).unwrap())
        }
    }
}

impl PartialOrd for DateTuple {
    fn partial_cmp(&self, other: &DateTuple) -> Option<Ordering> {
        if self.y == other.y {
            if self.m == other.m {
                self.d.partial_cmp(&other.d)
            } else {
                self.m.partial_cmp(&other.m)
            }
        } else {
            self.y.partial_cmp(&other.y)
        }
    }
}

impl Ord for DateTuple {
    fn cmp(&self, other: &DateTuple) -> Ordering {
        if self.y == other.y {
            if self.m == other.m {
                self.d.cmp(&other.d)
            } else {
                self.m.cmp(&other.m)
            }
        } else {
            self.y.cmp(&other.y)
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_to_string() {
        let tuple = super::DateTuple::new(2000, 5, 10).unwrap();
        assert_eq!(String::from("20000510"), tuple.to_string());
    }

    #[test]
    fn test_to_readable_string() {
        let tuple = super::DateTuple::new(2000, 5, 10).unwrap();
        assert_eq!(String::from("10 Jun 2000"), tuple.to_readable_string());
    }

    #[test]
    fn test_equals() {
        let tuple1 = super::DateTuple::new(2000, 5, 10).unwrap();
        let tuple2 = super::DateTuple::new(2000, 5, 10).unwrap();
        assert_eq!(tuple1, tuple2);
    }

    #[test]
    fn test_comparisons() {
        let tuple1 = super::DateTuple::new(2000, 5, 5).unwrap();
        let tuple2 = super::DateTuple::new(2000, 5, 5).unwrap();
        let tuple3 = super::DateTuple::new(2000, 6, 4).unwrap();
        let tuple4 = super::DateTuple::new(2001, 0, 1).unwrap();
        assert!(tuple1 <= tuple2);
        assert!(!(tuple1 < tuple2));
        assert!(tuple1 >= tuple2);
        assert!(tuple1 < tuple3);
        assert!(tuple3 < tuple4);
        assert!(tuple4 > tuple2);
    }

    #[test]
    fn test_validity() {
        let valid = [
            super::DateTuple::new(2000, 5, 5),
            super::DateTuple::new(2000, 6, 31),
            super::DateTuple::new(2000, 1, 2),
            super::DateTuple::new(2000, 1, 29),
        ];
        let invalid = [
            super::DateTuple::new(2000, 5, 31),
            super::DateTuple::new(2001, 1, 29),
            super::DateTuple::new(2000, 12, 5),
        ];
        for v in valid.iter() {
            if let Err(_) = v {
                assert!(false);
            }
        }
        for i in invalid.iter() {
            if let Ok(_) = i {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_from_string() {
        let tuple = super::DateTuple::new(2000, 5, 10).unwrap();
        assert_eq!(tuple, str::parse("20000510").unwrap());
    }

}

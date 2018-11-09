use date_tuple::DateTuple;
use regex::Regex;
use std::cmp::Ordering;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

const MONTH_STRINGS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub type Month = MonthTuple;

/// A container for a month of a specific year.
///
/// **NOTE:** MonthTuple's `m` field is zero-based (zero represents January).
///
/// Only handles values between Jan 0000 and Dec 9999 (inclusive).
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct MonthTuple {
    y: u16,
    m: u8,
}

impl MonthTuple {
    /// Produces a new MonthTuple.
    ///
    /// Only accepts a valid month value (`0 <= m <= 11`).
    ///
    /// Only accepts a valid year value (`0 <= y <= 9999`).
    pub fn new(y: u16, m: u8) -> Result<MonthTuple, String> {
        if m <= 11 {
            if y <= 9999 {
                Ok(MonthTuple { y, m })
            } else {
                Err(format!(
                    "Invalid year in MonthTuple: {:?}\nYear must be <= 9999.",
                    MonthTuple { y, m }
                ))
            }
        } else {
            Err(format!(
                "Invalid month in MonthTuple: {:?}\nMonth must be <= 11; Note that months are ZERO-BASED.",
                MonthTuple { y, m }
            ))
        }
    }

    pub fn get_year(&self) -> u16 {
        self.y
    }

    /// Retrieves the month component of the tuple.
    ///
    /// Note this month is **ZERO-BASED** (zero represents January).
    pub fn get_month(&self) -> u8 {
        self.m
    }

    /// Gets a MonthTuple representing the month immediately following
    /// the current one. Will not go past Dec 9999.
    pub fn next_month(self) -> MonthTuple {
        if self.y == 9999 && self.m == 11 {
            return self;
        }
        if self.m == 11 {
            MonthTuple {
                y: self.y + 1,
                m: 0,
            }
        } else {
            MonthTuple {
                y: self.y,
                m: self.m + 1,
            }
        }
    }

    /// Gets a MonthTuple representing the month immediately preceding
    /// the current one. Will not go past Jan 0000.
    pub fn previous_month(self) -> MonthTuple {
        if self.y == 0 && self.m == 0 {
            return self;
        }
        if self.m == 0 {
            MonthTuple {
                y: self.y - 1,
                m: 11,
            }
        } else {
            MonthTuple {
                y: self.y,
                m: self.m - 1,
            }
        }
    }

    /// Adds a number of months to a MonthTuple.
    pub fn add_months(&mut self, months: u32) {
        for _ in 0..months {
            *self = self.next_month();
        }
    }

    /// Returns the month formatted to be human-readable.
    ///
    /// ## Examples
    /// * Jan 2018
    /// * Dec 1994
    pub fn to_readable_string(&self) -> String {
        match MONTH_STRINGS.iter().skip(self.m as usize).next() {
            Some(s) => return format!("{} {:04}", s, self.y),
            None => panic!("Invalid MonthTuple: {:?}", self),
        }
    }
}

impl fmt::Display for MonthTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}{:02}", self.y, self.m)
    }
}

impl FromStr for MonthTuple {
    type Err = String;

    fn from_str(s: &str) -> Result<MonthTuple, Self::Err> {
        let valid_format = Regex::new(r"^\d{6}$").unwrap();
        if !valid_format.is_match(s) {
            Err(format!(
                "Invalid str formatting of MonthTuple: {}\nExpects a string formatted like 201811",
                s
            ))
        } else {
            let (s1, s2) = s.split_at(4);
            Ok(MonthTuple::new(u16::from_str(s1).unwrap(), u8::from_str(s2).unwrap()).unwrap())
        }
    }
}

impl PartialOrd for MonthTuple {
    fn partial_cmp(&self, other: &MonthTuple) -> Option<Ordering> {
        u32::from_str(&self.to_string())
            .unwrap()
            .partial_cmp(&u32::from_str(&other.to_string()).unwrap())
    }
}

impl Ord for MonthTuple {
    fn cmp(&self, other: &MonthTuple) -> Ordering {
        u32::from_str(&self.to_string())
            .unwrap()
            .cmp(&u32::from_str(&other.to_string()).unwrap())
    }
}

impl From<DateTuple> for MonthTuple {
    fn from(date: DateTuple) -> Self {
        MonthTuple {
            y: date.get_year(),
            m: date.get_month(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_next_month() {
        let tuple1 = super::MonthTuple::new(2000, 5).unwrap();
        let tuple2 = super::MonthTuple::new(2000, 11).unwrap();
        assert_eq!(super::MonthTuple { y: 2000, m: 6 }, tuple1.next_month());
        assert_eq!(super::MonthTuple { y: 2001, m: 0 }, tuple2.next_month());
    }

    #[test]
    fn test_previous_month() {
        let tuple1 = super::MonthTuple::new(2000, 5).unwrap();
        let tuple2 = super::MonthTuple::new(2000, 0).unwrap();
        assert_eq!(super::MonthTuple { y: 2000, m: 4 }, tuple1.previous_month());
        assert_eq!(
            super::MonthTuple { y: 1999, m: 11 },
            tuple2.previous_month()
        );
    }

    #[test]
    fn test_to_readable_string() {
        let tuple = super::MonthTuple::new(2000, 5).unwrap();
        assert_eq!(String::from("Jun 2000"), tuple.to_readable_string());
    }

    #[test]
    fn test_to_string() {
        let tuple = super::MonthTuple::new(2000, 5).unwrap();
        assert_eq!(String::from("200005"), tuple.to_string());
    }

    #[test]
    fn test_equals() {
        let tuple1 = super::MonthTuple::new(2000, 5).unwrap();
        let tuple2 = super::MonthTuple::new(2000, 5).unwrap();
        assert_eq!(tuple1, tuple2);
    }

    #[test]
    fn test_comparisons() {
        let tuple1 = super::MonthTuple::new(2000, 5).unwrap();
        let tuple2 = super::MonthTuple::new(2000, 5).unwrap();
        let tuple3 = super::MonthTuple::new(2000, 6).unwrap();
        let tuple4 = super::MonthTuple::new(2001, 0).unwrap();
        assert!(tuple1 <= tuple2);
        assert!(!(tuple1 < tuple2));
        assert!(tuple1 >= tuple2);
        assert!(tuple1 < tuple3);
        assert!(tuple3 < tuple4);
        assert!(tuple4 > tuple2);
    }

    #[test]
    fn test_invalid() {
        if let Ok(_) = super::MonthTuple::new(2000, 12) {
            assert!(false)
        }
    }

    #[test]
    fn test_from_date() {
        let date = ::date_tuple::DateTuple::new(2000, 5, 10).unwrap();
        assert_eq!(
            super::MonthTuple { y: 2000, m: 5 },
            super::MonthTuple::from(date)
        );
    }

    #[test]
    fn test_from_string() {
        let tuple = super::MonthTuple::new(2000, 5).unwrap();
        assert_eq!(tuple, str::parse("200005").unwrap());
    }

    #[test]
    fn test_add_months() {
        let mut tuple1 = super::MonthTuple::new(2000, 5).unwrap();
        let tuple1_orig = super::MonthTuple::new(2000, 5).unwrap();
        let mut tuple2 = super::MonthTuple::new(2000, 11).unwrap();
        let tuple2_orig = super::MonthTuple::new(2000, 11).unwrap();
        tuple1.add_months(1);
        assert_eq!(tuple1, tuple1_orig.next_month());
        tuple2.add_months(2);
        assert_eq!(tuple2, tuple2_orig.next_month().next_month());
    }

}

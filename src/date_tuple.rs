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
            if d == 0 || d > get_last_date_in_month(m, y) {
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

    /// Returns a `DateTuple` of the current date according to the system clock.
    pub fn today() -> DateTuple {
        date_utils::now_as_datetuple()
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

    /// Gets a DateTuple representing the date immediately following
    /// the current one. Will not go past Dec 9999.
    pub fn next_date(self) -> DateTuple {
        if self.y == 9999 && self.m == 11 && self.d == 31 {
            return self;
        }
        if self.d == get_last_date_in_month(self.m, self.y) {
            if self.m == 11 {
                return DateTuple {
                    y: self.y + 1,
                    m: 0,
                    d: 1,
                };
            } else {
                return DateTuple {
                    y: self.y,
                    m: self.m + 1,
                    d: 1,
                };
            }
        } else {
            return DateTuple {
                y: self.y,
                m: self.m,
                d: self.d + 1,
            };
        }
    }

    /// Gets a DateTuple representing the date immediately preceding
    /// the current one. Will not go past 1 Jan 0000.
    pub fn previous_date(self) -> DateTuple {
        if self.y == 0 && self.m == 0 && self.d == 1 {
            return self;
        }
        if self.d == 1 {
            if self.m == 0 {
                return DateTuple {
                    y: self.y - 1,
                    m: 11,
                    d: get_last_date_in_month(11, self.y - 1),
                };
            } else {
                return DateTuple {
                    y: self.y,
                    m: self.m - 1,
                    d: get_last_date_in_month(self.m - 1, self.y),
                };
            }
        } else {
            return DateTuple {
                y: self.y,
                m: self.m,
                d: self.d - 1,
            };
        }
    }

    /// Adds a number of days to a DateTuple.
    pub fn add_days(&mut self, days: u32) {
        for _ in 0..days {
            *self = self.next_date();
        }
    }

    /// Subtracts a number of days from a DateTuple.
    pub fn subtract_days(&mut self, days: u32) {
        for _ in 0..days {
            *self = self.previous_date();
        }
    }

    /// Adds a number of months to a DateTuple.
    ///
    /// If the day of month is beyond the last date in the resulting month, the day of
    /// month will be set to the last day of that month.
    pub fn add_months(&mut self, months: u32) {
        let mut new_month = MonthTuple::from(*self);
        new_month.add_months(months);
        let last_date_in_month =
            get_last_date_in_month(new_month.get_month(), new_month.get_year());
        if self.d > last_date_in_month {
            self.d = last_date_in_month;
        }
        self.y = new_month.get_year();
        self.m = new_month.get_month();
    }

    /// Subtracts a number of months from a DateTuple.
    ///
    /// If the day of month is beyond the last date in the resulting month, the day of
    /// month will be set to the last day of that month.
    pub fn subtract_months(&mut self, months: u32) {
        let mut new_month = MonthTuple::from(*self);
        new_month.subtract_months(months);
        let last_date_in_month =
            get_last_date_in_month(new_month.get_month(), new_month.get_year());
        if self.d > last_date_in_month {
            self.d = last_date_in_month;
        }
        self.y = new_month.get_year();
        self.m = new_month.get_month();
    }

    /// Adds a number of years to a DateTuple.
    ///
    /// If the date is set to Feb 29 and the resulting year is not a leap year,
    /// it will be changed to Feb 28.
    pub fn add_years(&mut self, years: u16) {
        let mut new_years = self.y + years;
        if new_years > 9999 {
            new_years = 9999;
        }
        if self.m == 1 && self.d == 29 && !date_utils::is_leap_year(new_years) {
            self.d = 28
        }
        self.y = new_years;
    }

    /// Subtracts a number of years from a DateTuple.
    ///
    /// If the date is set to Feb 29 and the resulting year is not a leap year,
    /// it will be changed to Feb 28.
    pub fn subtract_years(&mut self, years: u16) {
        let mut new_years = self.y as i32 - years as i32;
        if new_years < 0 {
            new_years = 0;
        }
        let new_years = new_years as u16;
        if self.m == 1 && self.d == 29 && !date_utils::is_leap_year(new_years) {
            self.d = 28
        }
        self.y = new_years;
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

/// Produces the integer representing the last date in the **ZERO-BASED**
/// month in year.
fn get_last_date_in_month(month: u8, year: u16) -> u8 {
    match month {
        1 => {
            if date_utils::is_leap_year(year) {
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
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_year_too_large() {
        assert!(super::DateTuple::new(10000, 5, 10).is_err());
    }

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
        assert!(str::parse::<super::DateTuple>("2O00051O").is_err());
    }

    #[test]
    fn test_next_date() {
        let tuple1 = super::Date::new(2000, 5, 10).unwrap();
        let tuple2 = super::Date::new(2000, 2, 31).unwrap();
        let tuple3 = super::Date::new(9999, 11, 31).unwrap();
        assert_eq!(
            super::Date {
                y: 2000,
                m: 5,
                d: 11
            },
            tuple1.next_date()
        );
        assert_eq!(
            super::Date {
                y: 2000,
                m: 3,
                d: 1
            },
            tuple2.next_date()
        );
        assert_eq!(
            super::Date {
                y: 9999,
                m: 11,
                d: 31
            },
            tuple3.next_date()
        );
    }

    #[test]
    fn test_previous_date() {
        let tuple1 = super::Date::new(2000, 5, 10).unwrap();
        let tuple2 = super::Date::new(2000, 2, 1).unwrap();
        let tuple3 = super::Date::new(0, 0, 1).unwrap();
        let tuple4 = super::Date::new(2000, 0, 1).unwrap();
        assert_eq!(
            super::Date {
                y: 2000,
                m: 5,
                d: 9
            },
            tuple1.previous_date()
        );
        assert_eq!(
            super::Date {
                y: 2000,
                m: 1,
                d: 29
            },
            tuple2.previous_date()
        );
        assert_eq!(super::Date { y: 0, m: 0, d: 1 }, tuple3.previous_date());
        assert_eq!(
            super::Date {
                y: 1999,
                m: 11,
                d: 31
            },
            tuple4.previous_date()
        );
    }

    #[test]
    fn test_add_days() {
        let mut tuple1 = super::DateTuple::new(2000, 5, 5).unwrap();
        let tuple1_orig = super::DateTuple::new(2000, 5, 5).unwrap();
        let mut tuple2 = super::DateTuple::new(2000, 11, 31).unwrap();
        let tuple2_orig = super::DateTuple::new(2000, 11, 31).unwrap();
        tuple1.add_days(1);
        assert_eq!(tuple1, tuple1_orig.next_date());
        tuple2.add_days(2);
        assert_eq!(tuple2, tuple2_orig.next_date().next_date());
    }

    #[test]
    fn test_subtract_days() {
        let mut tuple1 = super::DateTuple::new(2000, 5, 5).unwrap();
        let tuple1_orig = super::DateTuple::new(2000, 5, 5).unwrap();
        let mut tuple2 = super::DateTuple::new(2000, 11, 31).unwrap();
        let tuple2_orig = super::DateTuple::new(2000, 11, 31).unwrap();
        tuple1.subtract_days(1);
        assert_eq!(tuple1, tuple1_orig.previous_date());
        tuple2.subtract_days(2);
        assert_eq!(tuple2, tuple2_orig.previous_date().previous_date());
    }

    #[test]
    fn test_add_months() {
        let mut tuple1 = super::DateTuple::new(2000, 5, 1).unwrap();
        let mut tuple2 = super::DateTuple::new(2000, 6, 31).unwrap();
        tuple1.add_months(1);
        assert_eq!(tuple1, super::DateTuple::new(2000, 6, 1).unwrap());
        tuple1.add_months(1);
        assert_eq!(tuple1, super::DateTuple::new(2000, 7, 1).unwrap());
        tuple2.add_months(2);
        assert_eq!(tuple2, super::DateTuple::new(2000, 8, 30).unwrap());
    }

    #[test]
    fn test_subtract_months() {
        let mut tuple1 = super::DateTuple::new(2000, 5, 1).unwrap();
        let mut tuple2 = super::DateTuple::new(2000, 6, 31).unwrap();
        let mut tuple3 = super::DateTuple::new(2000, 10, 30).unwrap();
        tuple1.subtract_months(1);
        assert_eq!(tuple1, super::DateTuple::new(2000, 4, 1).unwrap());
        tuple2.subtract_months(3);
        assert_eq!(tuple2, super::DateTuple::new(2000, 3, 30).unwrap());
        tuple3.subtract_months(1);
        assert_eq!(tuple3, super::DateTuple::new(2000, 9, 30).unwrap());
    }

    #[test]
    fn test_add_and_subtract_years() {
        let mut tuple1 = super::Date::new(2000, 1, 29).unwrap();
        let mut tuple2 = super::Date::new(2000, 1, 29).unwrap();
        tuple1.add_years(1);
        tuple2.add_years(4);
        assert_eq!(super::Date::new(2001, 1, 28).unwrap(), tuple1);
        assert_eq!(super::Date::new(2004, 1, 29).unwrap(), tuple2);
        tuple1.subtract_years(1);
        tuple2.subtract_years(4);
        assert_eq!(super::Date::new(2000, 1, 28).unwrap(), tuple1);
        assert_eq!(super::Date::new(2000, 1, 29).unwrap(), tuple2);
        tuple2.subtract_years(1);
        assert_eq!(super::Date::new(1999, 1, 28).unwrap(), tuple2);
        let mut tuple3 = super::Date::new(9999, 5, 10).unwrap();
        let mut tuple4 = super::Date::new(0, 5, 10).unwrap();
        tuple3.add_years(1);
        tuple4.subtract_years(1);
        assert_eq!(9999, tuple3.get_year());
        assert_eq!(0, tuple4.get_year());
    }

}

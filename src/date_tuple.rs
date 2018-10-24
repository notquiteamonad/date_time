use date_utils;
use month_tuple::MonthTuple;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct DateTuple {
    y: u32,
    m: u32,
    d: u32,
}

impl DateTuple {
    pub fn new(y: u32, m: u32, d: u32) -> Result<DateTuple, String> {
        if m <= 11 {
            let max_date = match m {
                1 => {
                    if date_utils::is_leap_year(y) {
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
            Ok(DateTuple { y, m, d })
        } else {
            Err(format!(
                "Invalid month in DateTuple: {:?}\nMonth must be <= 11; Note that months are ZERO-BASED.",
                DateTuple { y, m, d }
            ))
        }
    }

    pub fn get_year(&self) -> u32 {
        self.y
    }

    pub fn get_month(&self) -> u32 {
        self.m
    }

    pub fn get_date(&self) -> u32 {
        self.d
    }

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

}

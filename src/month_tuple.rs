use date_tuple::DateTuple;
use regex::Regex;
use std::cmp::Ordering;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

const MONTH_STRINGS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/**
 * **NOTE:** MonthTuple's `m` field is zero-based (zero represents January).
 */
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct MonthTuple {
    y: u32,
    m: u32,
}

impl MonthTuple {
    pub fn new(y: u32, m: u32) -> Result<MonthTuple, String> {
        if m <= 11 {
            Ok(MonthTuple { y, m })
        } else {
            Err(format!(
                "Invalid month in MonthTuple: {:?}\nMonth must be <= 11; Note that months are ZERO-BASED.",
                MonthTuple { y, m }
            ))
        }
    }

    pub fn get_year(&self) -> u32 {
        self.y
    }

    /**
     * Note this month is **ZERO-BASED** (zero represents January).
     */
    pub fn get_month(&self) -> u32 {
        self.m
    }

    pub fn next_month(self) -> MonthTuple {
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

    pub fn previous_month(self) -> MonthTuple {
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
            Err(format!("Invalid str formatting of MonthTuple: {}", s))
        } else {
            let (s1, s2) = s.split_at(4);
            Ok(MonthTuple::new(u32::from_str(s1).unwrap(), u32::from_str(s2).unwrap()).unwrap())
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

}

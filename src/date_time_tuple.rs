use date_tuple::DateTuple;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use time_tuple::TimeTuple;

pub type DateTime = DateTimeTuple;

/// Wrapper for a specific date and time.
///
/// Comprised of a DateTuple and a TimeTuple.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct DateTimeTuple {
    d: DateTuple,
    t: TimeTuple,
}

impl DateTimeTuple {
    pub fn new(d: DateTuple, t: TimeTuple) -> DateTimeTuple {
        DateTimeTuple { d, t }
    }

    pub fn get_date(self) -> DateTuple {
        self.d
    }

    pub fn get_time(self) -> TimeTuple {
        self.t
    }

    /// Produces a readable date and time.
    ///
    /// ## Examples
    /// * 2 Oct 2018 08:30:00
    /// * 13 Jan 2019 11:00:10
    pub fn to_readable_string(self) -> String {
        format!("{} {}", self.d.to_readable_string(), self.t.to_string())
    }
}

/// Gets a string to to use for storage. This string can be interpreted
/// by `str::parse`.
///
/// Formatted like 2018-10-02@08:30:00
impl fmt::Display for DateTimeTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.d.to_string(), self.t.to_string())
    }
}

impl FromStr for DateTimeTuple {
    type Err = String;

    /// Expects a string formatted like one obtained by calling `DateTimeTuple.to_string()`
    fn from_str(s: &str) -> Result<DateTimeTuple, Self::Err> {
        lazy_static! {
            static ref VALID_FORMAT: Regex =
                Regex::new(r"^\d{4}-\d{2}-\d{2}@\d{2}:\d{2}:\d{2}$").unwrap();
            static ref LEGACY_FORMAT: Regex = Regex::new(r"^\d{8}@\d{2}:\d{2}:\d{2}$").unwrap();
        }

        if VALID_FORMAT.is_match(s) || LEGACY_FORMAT.is_match(s) {
            let mut parts = s.split('@');
            let date_part = match DateTuple::from_str(parts.next().unwrap()) {
                Ok(d) => d,
                Err(e) => return Err(format!("Invalid date passed to from_str: {}", e)),
            };
            let time_part = TimeTuple::from_str(parts.next().unwrap()).unwrap();
            Ok(DateTimeTuple::new(date_part, time_part))
        } else {
            Err(format!("Invalid str formatting of DateTimeTuple: {}\nExpects a string formatted like 2018-11-02@08:30:00", s))
        }
    }
}

impl PartialOrd for DateTimeTuple {
    fn partial_cmp(&self, other: &DateTimeTuple) -> Option<Ordering> {
        if self.d == other.d {
            self.t.partial_cmp(&other.t)
        } else {
            self.d.partial_cmp(&other.d)
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Ord for DateTimeTuple {
    fn cmp(&self, other: &DateTimeTuple) -> Ordering {
        if self.d == other.d {
            self.t.cmp(&other.t)
        } else {
            self.d.cmp(&other.d)
        }
    }
}

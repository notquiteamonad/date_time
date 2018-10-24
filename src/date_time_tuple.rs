use date_tuple::DateTuple;
use std::cmp::Ordering;
use std::fmt;
use time_tuple::TimeTuple;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct DateTimeTuple {
    d: DateTuple,
    t: TimeTuple,
}

impl DateTimeTuple {
    pub fn new(d: DateTuple, t: TimeTuple) -> DateTimeTuple {
        DateTimeTuple { d, t }
    }

    pub fn get_date(&self) -> DateTuple {
        self.d
    }

    pub fn get_time(&self) -> TimeTuple {
        self.t
    }

    pub fn to_readable_string(&self) -> String {
        format!("{} {}", self.d.to_readable_string(), self.t.to_string())
    }
}

impl fmt::Display for DateTimeTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.d.to_string(), self.t.to_string())
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

impl Ord for DateTimeTuple {
    fn cmp(&self, other: &DateTimeTuple) -> Ordering {
        if self.d == other.d {
            self.t.cmp(&other.t)
        } else {
            self.d.cmp(&other.d)
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_to_string() {
        let tuple = super::DateTimeTuple::new(
            ::date_tuple::DateTuple::new(2000, 5, 10).unwrap(),
            ::time_tuple::TimeTuple::new(8, 30, 0),
        );
        assert_eq!(String::from("20000510@08:30:00"), tuple.to_string());
    }

    #[test]
    fn test_to_readable_string() {
        let tuple = super::DateTimeTuple::new(
            ::date_tuple::DateTuple::new(2000, 5, 10).unwrap(),
            ::time_tuple::TimeTuple::new(8, 30, 0),
        );
        assert_eq!(
            String::from("10 Jun 2000 08:30:00"),
            tuple.to_readable_string()
        );
    }

    #[test]
    fn test_equals() {
        let tuple1 = super::DateTimeTuple::new(
            ::date_tuple::DateTuple::new(2000, 5, 10).unwrap(),
            ::time_tuple::TimeTuple::new(8, 30, 0),
        );
        let tuple2 = super::DateTimeTuple::new(
            ::date_tuple::DateTuple::new(2000, 5, 10).unwrap(),
            ::time_tuple::TimeTuple::new(8, 30, 0),
        );
        assert_eq!(tuple1, tuple2);
    }

    #[test]
    fn test_comparisons() {
        let tuple1 = super::DateTimeTuple::new(
            ::date_tuple::DateTuple::new(2000, 5, 10).unwrap(),
            ::time_tuple::TimeTuple::new(8, 30, 0),
        );
        let tuple2 = super::DateTimeTuple::new(
            ::date_tuple::DateTuple::new(2000, 5, 10).unwrap(),
            ::time_tuple::TimeTuple::new(9, 30, 0),
        );
        let tuple3 = super::DateTimeTuple::new(
            ::date_tuple::DateTuple::new(2000, 5, 11).unwrap(),
            ::time_tuple::TimeTuple::new(8, 30, 0),
        );
        assert!(tuple1 < tuple2);
        assert!(tuple2 < tuple3);
    }

}

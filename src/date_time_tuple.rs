use date_tuple::DateTuple;
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

}

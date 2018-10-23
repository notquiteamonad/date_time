use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct MonthTuple {
    y: u32,
    m: u32,
}

impl fmt::Display for MonthTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}{:02}", self.y, self.m)
    }
}

#[cfg(test)]
mod tests {

    //todo nextmonth
    //todo previousmonth
    //todo toreadablestring

    #[test]
    fn test_to_string() {
        let tuple = super::MonthTuple { y: 2000, m: 5 };
        assert_eq!(String::from("200005"), tuple.to_string());
    }

    #[test]
    fn test_equals() {
        let tuple1 = super::MonthTuple { y: 2000, m: 5 };
        let tuple2 = super::MonthTuple { y: 2000, m: 5 };
        assert_eq!(tuple1, tuple2);
    }

    //todo compareto

}

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/**
 * **NOTE:** MonthTuple's `m` field is zero-based (zero represents January).
 */
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

    #[test]
    fn test_comparisons() {
        let tuple1 = super::MonthTuple { y: 2000, m: 5 };
        let tuple2 = super::MonthTuple { y: 2000, m: 5 };
        let tuple3 = super::MonthTuple { y: 2000, m: 6 };
        let tuple4 = super::MonthTuple { y: 2001, m: 0 };
        assert!(tuple1 <= tuple2);
        assert!(!(tuple1 < tuple2));
        assert!(tuple1 >= tuple2);
        assert!(tuple1 < tuple3);
        assert!(tuple3 < tuple4);
        assert!(tuple4 > tuple2);
    }

}

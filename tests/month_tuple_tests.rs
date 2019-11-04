extern crate date_time;

use date_time::month_tuple::MonthTuple;

#[test]
fn test_component_too_large() {
    assert!(MonthTuple::new(2000, 12).is_ok());
    assert!(MonthTuple::new(2000, 13).is_err());
    assert!(MonthTuple::new(10000, 5).is_err());
}

#[test]
fn test_this_month_does_not_panic() {
    MonthTuple::this_month();
}

#[test]
fn test_to_readable_string() {
    let tuple = MonthTuple::new(2000, 5).unwrap();
    assert_eq!(String::from("May 2000"), tuple.to_readable_string());
}

#[test]
fn test_to_string() {
    let tuple = MonthTuple::new(2000, 5).unwrap();
    assert_eq!(String::from("2000-05"), tuple.to_string());
}

#[test]
fn test_equals() {
    let tuple1 = MonthTuple::new(2000, 5).unwrap();
    let tuple2 = MonthTuple::new(2000, 5).unwrap();
    assert_eq!(tuple1, tuple2);
}

#[test]
fn test_comparisons() {
    let tuple1 = MonthTuple::new(2000, 5).unwrap();
    let tuple2 = MonthTuple::new(2000, 5).unwrap();
    let tuple3 = MonthTuple::new(2000, 6).unwrap();
    let tuple4 = MonthTuple::new(2001, 1).unwrap();
    assert!(tuple1 <= tuple2);
    assert!(!(tuple1 < tuple2));
    assert!(tuple1 >= tuple2);
    assert!(tuple1 < tuple3);
    assert!(tuple3 < tuple4);
    assert!(tuple4 > tuple2);
}

#[test]
fn test_from_string() {
    let tuple = MonthTuple::new(2000, 5).unwrap();
    assert_eq!(tuple, str::parse("2000-05").unwrap());
    assert_eq!(tuple, str::parse("200005").unwrap());
    assert!(str::parse::<MonthTuple>("2000-15").is_err());
    assert!(str::parse::<MonthTuple>("200015").is_err());
    assert!(str::parse::<MonthTuple>("200O05").is_err());
}

#[test]
fn test_add_months() {
    let mut tuple1 = MonthTuple::new(2000, 6).unwrap();
    let tuple1_orig = MonthTuple::new(2000, 6).unwrap();
    let mut tuple2 = MonthTuple::new(2000, 12).unwrap();
    let tuple2_orig = MonthTuple::new(2000, 12).unwrap();
    tuple1.add_months(1);
    assert_eq!(tuple1, tuple1_orig.next_month());
    tuple2.add_months(2);
    assert_eq!(tuple2, tuple2_orig.next_month().next_month());
}

#[test]
fn test_subtract_months() {
    let mut tuple1 = MonthTuple::new(2000, 6).unwrap();
    let tuple1_orig = MonthTuple::new(2000, 6).unwrap();
    let mut tuple2 = MonthTuple::new(2000, 12).unwrap();
    let tuple2_orig = MonthTuple::new(2000, 12).unwrap();
    tuple1.subtract_months(1);
    assert_eq!(tuple1, tuple1_orig.previous_month());
    tuple2.subtract_months(2);
    assert_eq!(tuple2, tuple2_orig.previous_month().previous_month());
}

#[test]
fn test_add_years() {
    let mut tuple1 = MonthTuple::new(2000, 6).unwrap();
    let mut tuple2 = MonthTuple::new(9998, 6).unwrap();
    tuple1.add_years(2);
    assert_eq!(2002, tuple1.get_year());
    tuple2.add_years(2);
    assert_eq!(9999, tuple2.get_year());
}

#[test]
fn test_subtract_years() {
    let mut tuple1 = MonthTuple::new(2000, 6).unwrap();
    let mut tuple2 = MonthTuple::new(1, 6).unwrap();
    tuple1.subtract_years(2);
    assert_eq!(1998, tuple1.get_year());
    tuple2.subtract_years(2);
    assert_eq!(0, tuple2.get_year());
}

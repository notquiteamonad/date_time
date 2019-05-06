extern crate date_time;

use date_time::date_tuple::{Date, DateTuple};

#[test]
fn test_year_too_large() {
    assert!(DateTuple::new(10000, 6, 10).is_err());
}

#[test]
fn test_to_string() {
    let tuple = DateTuple::new(2000, 6, 10).unwrap();
    assert_eq!(String::from("2000-06-10"), tuple.to_string());
}

#[test]
fn test_to_readable_string() {
    let tuple = DateTuple::new(2000, 6, 10).unwrap();
    assert_eq!(String::from("10 Jun 2000"), tuple.to_readable_string());
}

#[test]
fn test_equals() {
    let tuple1 = DateTuple::new(2000, 6, 10).unwrap();
    let tuple2 = DateTuple::new(2000, 6, 10).unwrap();
    assert_eq!(tuple1, tuple2);
}

#[test]
fn test_comparisons() {
    let tuple1 = DateTuple::new(2000, 6, 5).unwrap();
    let tuple2 = DateTuple::new(2000, 6, 5).unwrap();
    let tuple3 = DateTuple::new(2000, 7, 4).unwrap();
    let tuple4 = DateTuple::new(2001, 1, 1).unwrap();
    assert!(tuple1 <= tuple2);
    assert!(!(tuple1 < tuple2));
    assert!(tuple1 >= tuple2);
    assert!(tuple1 < tuple3);
    assert!(tuple3 < tuple4);
    assert!(tuple4 > tuple2);
}

#[test]
fn test_validity() {
    assert!(DateTuple::new(2000, 6, 5).is_ok());
    assert!(DateTuple::new(2000, 7, 31).is_ok());
    assert!(DateTuple::new(2000, 2, 2).is_ok());
    assert!(DateTuple::new(2000, 2, 29).is_ok());

    assert!(DateTuple::new(2000, 6, 31).is_err());
    assert!(DateTuple::new(2001, 2, 29).is_err());
    assert!(DateTuple::new(2000, 13, 5).is_err());
}

#[test]
fn test_from_string() {
    let tuple = DateTuple::new(2000, 6, 10).unwrap();
    assert_eq!(tuple, str::parse("2000-06-10").unwrap());
    assert!(str::parse::<DateTuple>("2000-16-10").is_err());
    assert!(str::parse::<DateTuple>("2O00061O").is_err());
}

#[test]
fn test_from_legacy_string() {
    let tuple = DateTuple::new(2000, 6, 10).unwrap();
    assert_eq!(tuple, str::parse("20000610").unwrap());
    assert!(str::parse::<DateTuple>("20001610").is_err());
}

#[test]
fn test_add_days() {
    let mut tuple1 = DateTuple::new(2000, 6, 5).unwrap();
    let tuple1_orig = DateTuple::new(2000, 6, 5).unwrap();
    let mut tuple2 = DateTuple::new(2000, 12, 31).unwrap();
    let tuple2_orig = DateTuple::new(2000, 12, 31).unwrap();
    tuple1.add_days(1);
    assert_eq!(tuple1, tuple1_orig.next_date());
    tuple2.add_days(2);
    assert_eq!(tuple2, tuple2_orig.next_date().next_date());
}

#[test]
fn test_subtract_days() {
    let mut tuple1 = DateTuple::new(2000, 6, 5).unwrap();
    let tuple1_orig = DateTuple::new(2000, 6, 5).unwrap();
    let mut tuple2 = DateTuple::new(2000, 12, 31).unwrap();
    let tuple2_orig = DateTuple::new(2000, 12, 31).unwrap();
    tuple1.subtract_days(1);
    assert_eq!(tuple1, tuple1_orig.previous_date());
    tuple2.subtract_days(2);
    assert_eq!(tuple2, tuple2_orig.previous_date().previous_date());
}

#[test]
fn test_add_months() {
    let mut tuple1 = DateTuple::new(2000, 6, 1).unwrap();
    let mut tuple2 = DateTuple::new(2000, 7, 31).unwrap();
    tuple1.add_months(1);
    assert_eq!(tuple1, DateTuple::new(2000, 7, 1).unwrap());
    tuple1.add_months(1);
    assert_eq!(tuple1, DateTuple::new(2000, 8, 1).unwrap());
    tuple2.add_months(2);
    assert_eq!(tuple2, DateTuple::new(2000, 9, 30).unwrap());
}

#[test]
fn test_subtract_months() {
    let mut tuple1 = DateTuple::new(2000, 6, 1).unwrap();
    let mut tuple2 = DateTuple::new(2000, 7, 31).unwrap();
    let mut tuple3 = DateTuple::new(2000, 11, 30).unwrap();
    tuple1.subtract_months(1);
    assert_eq!(tuple1, DateTuple::new(2000, 5, 1).unwrap());
    tuple2.subtract_months(3);
    assert_eq!(tuple2, DateTuple::new(2000, 4, 30).unwrap());
    tuple3.subtract_months(1);
    assert_eq!(tuple3, DateTuple::new(2000, 10, 30).unwrap());
}

#[test]
fn test_add_and_subtract_years() {
    let mut tuple1 = Date::new(2000, 2, 29).unwrap();
    let mut tuple2 = Date::new(2000, 2, 29).unwrap();
    tuple1.add_years(1);
    tuple2.add_years(4);
    assert_eq!(Date::new(2001, 2, 28).unwrap(), tuple1);
    assert_eq!(Date::new(2004, 2, 29).unwrap(), tuple2);
    tuple1.subtract_years(1);
    tuple2.subtract_years(4);
    assert_eq!(Date::new(2000, 2, 28).unwrap(), tuple1);
    assert_eq!(Date::new(2000, 2, 29).unwrap(), tuple2);
    tuple2.subtract_years(1);
    assert_eq!(Date::new(1999, 2, 28).unwrap(), tuple2);
    let mut tuple3 = Date::new(9999, 6, 10).unwrap();
    let mut tuple4 = Date::new(0, 6, 10).unwrap();
    tuple3.add_years(1);
    tuple4.subtract_years(1);
    assert_eq!(9999, tuple3.get_year());
    assert_eq!(0, tuple4.get_year());
}

#[test]
fn test_to_days() {
    let feb_29_2000 = DateTuple::new(2000, 2, 29).unwrap();
    assert_eq!(730545, feb_29_2000.to_days());
}

#[test]
fn test_from_days() {
    let feb_29_2000 = DateTuple::new(2000, 2, 29).unwrap();
    assert_eq!(feb_29_2000, DateTuple::from_days(730545).unwrap());
    assert!(DateTuple::from_days(0).is_err());
}

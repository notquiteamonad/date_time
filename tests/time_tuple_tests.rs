extern crate date_time;

use date_time::time_tuple::{Duration, Time, TimeTuple};

#[test]
fn test_to_string() {
    let tuple = TimeTuple::new(3, 0, 39);
    assert_eq!(String::from("03:00:39"), tuple.to_string())
}

#[test]
fn test_to_hhmm_string() {
    let tuple = TimeTuple::new(3, 0, 39);
    assert_eq!(String::from("03:00"), tuple.to_hhmm_string())
}

#[test]
fn test_operators() {
    let zeroes = TimeTuple::new(0, 0, 0);
    let ones = TimeTuple::new(1, 1, 1);
    let twos = TimeTuple::new(2, 2, 2);
    assert_eq!(twos, ones + ones);
    assert_eq!(zeroes, ones - ones);
    assert!(zeroes < ones);
    assert!(zeroes < twos);
    assert!(twos > ones);
    assert!(zeroes <= ones);
    assert!(ones <= ones);
}

#[test]
fn test_to_seconds() {
    let tuple = TimeTuple::new(2, 30, 30);
    assert_eq!(9030, tuple.to_seconds());
}

#[test]
fn test_to_minutes() {
    let tuple = TimeTuple::new(2, 30, 30);
    let duration = Duration::new(26, 30, 30);
    assert_eq!(150, tuple.to_minutes());
    assert_eq!(1590, duration.to_minutes());
}

#[test]
fn test_from_seconds() {
    let tuple = TimeTuple::from_seconds(86400);
    assert_eq!(TimeTuple::new(0, 0, 0), tuple);
}

#[test]
fn test_from_string() {
    let tuple = TimeTuple::new(5, 30, 4);
    assert_eq!(tuple, str::parse("05:30:04").unwrap());
    assert!(str::parse::<TimeTuple>("05:a:04").is_err());
}

#[test]
fn test_manipulate_seconds() {
    let mut tuple = TimeTuple::new(10, 58, 59);
    tuple.add_seconds(3);
    assert_eq!(TimeTuple::new(10, 59, 2), tuple);
    tuple.subtract_seconds(1);
    tuple.subtract_seconds(2);
    assert_eq!(TimeTuple::new(10, 58, 59), tuple);
}

#[test]
fn test_manipulate_minutes() {
    let mut tuple = TimeTuple::new(10, 58, 59);
    tuple.add_minutes(3);
    assert_eq!(TimeTuple::new(11, 1, 59), tuple);
    tuple.subtract_minutes(1);
    tuple.subtract_minutes(2);
    assert_eq!(TimeTuple::new(10, 58, 59), tuple);
}

#[test]
fn test_manipulate_hours() {
    let mut tuple = TimeTuple::new(10, 58, 59);
    tuple.add_hours(3);
    assert_eq!(TimeTuple::new(13, 58, 59), tuple);
    tuple.subtract_hours(1);
    tuple.subtract_hours(2);
    assert_eq!(TimeTuple::new(10, 58, 59), tuple);
}

#[test]
fn test_time_to_duration() {
    let time = Time::new(20, 20, 20);
    let duration = Duration::from(time);
    assert_eq!(Duration::new(20, 20, 20), duration);
}

#[test]
fn test_large_duration() {
    let duration = Duration::new(200, 0, 0);
    assert_eq!(String::from("200:00:00"), duration.to_string());
}

extern crate date_time;

use date_time::date_time_tuple::DateTimeTuple;
use date_time::date_tuple::DateTuple;
use date_time::time_tuple::Duration;
use date_time::time_tuple::TimeTuple;

#[test]
fn test_to_string() {
    let tuple = DateTimeTuple::new(
        DateTuple::new(2000, 5, 10).unwrap(),
        TimeTuple::new(8, 30, 0),
    );
    assert_eq!(String::from("2000-05-10@08:30:00"), tuple.to_string());
}

#[test]
fn test_to_readable_string() {
    let tuple = DateTimeTuple::new(
        DateTuple::new(2000, 5, 10).unwrap(),
        TimeTuple::new(8, 30, 0),
    );
    assert_eq!(
        String::from("10 May 2000 08:30:00"),
        tuple.to_readable_string()
    );
}

#[test]
fn test_equals() {
    let tuple1 = DateTimeTuple::new(
        DateTuple::new(2000, 5, 10).unwrap(),
        TimeTuple::new(8, 30, 0),
    );
    let tuple2 = DateTimeTuple::new(
        DateTuple::new(2000, 5, 10).unwrap(),
        TimeTuple::new(8, 30, 0),
    );
    assert_eq!(tuple1, tuple2);
}

#[test]
fn test_comparisons() {
    let tuple1 = DateTimeTuple::new(
        DateTuple::new(2000, 5, 10).unwrap(),
        TimeTuple::new(8, 30, 0),
    );
    let tuple2 = DateTimeTuple::new(
        DateTuple::new(2000, 5, 10).unwrap(),
        TimeTuple::new(9, 30, 0),
    );
    let tuple3 = DateTimeTuple::new(
        DateTuple::new(2000, 5, 11).unwrap(),
        TimeTuple::new(8, 30, 0),
    );
    assert!(tuple1 < tuple2);
    assert!(tuple2 < tuple3);
}

#[test]
fn test_from_string() {
    let tuple = DateTimeTuple::new(
        DateTuple::new(2000, 5, 10).unwrap(),
        TimeTuple::new(8, 30, 0),
    );
    assert_eq!(tuple, str::parse("2000-05-10@08:30:00").unwrap());
    assert_eq!(tuple, str::parse("20000510@08:30:00").unwrap());
    assert!(str::parse::<DateTimeTuple>("2000-15-10@08:30:00").is_err());
    assert!(str::parse::<DateTimeTuple>("2-a11111@05:a:04").is_err());
}

#[test]
fn test_between_equal() {
    assert_eq!(
        Duration::new(0, 0, 0),
        Duration::between(
            DateTimeTuple::new(DateTuple::new(1, 2, 3).unwrap(), TimeTuple::new(4, 5, 6)),
            DateTimeTuple::new(DateTuple::new(1, 2, 3).unwrap(), TimeTuple::new(4, 5, 6))
        )
    );
}

#[test]
fn test_between_max_cant_overflow() {
    // Must not panic
    Duration::between(
        DateTimeTuple::new(DateTuple::new(0, 1, 1).unwrap(), TimeTuple::new(0, 0, 0)),
        DateTimeTuple::new(
            DateTuple::new(9999, 12, 31).unwrap(),
            TimeTuple::new(23, 59, 59),
        ),
    )
    .to_seconds();
}

#[test]
fn test_no_days_between() {
    assert_eq!(
        Duration::new(1, 0, 0),
        Duration::between(
            DateTimeTuple::new(DateTuple::new(0, 1, 1).unwrap(), TimeTuple::new(0, 0, 0)),
            DateTimeTuple::new(DateTuple::new(0, 1, 1).unwrap(), TimeTuple::new(1, 0, 0),),
        )
    );
}

#[test]
fn test_days_between() {
    assert_eq!(
        Duration::new(25, 0, 0),
        Duration::between(
            DateTimeTuple::new(DateTuple::new(0, 1, 1).unwrap(), TimeTuple::new(0, 0, 0)),
            DateTimeTuple::new(DateTuple::new(0, 1, 2).unwrap(), TimeTuple::new(1, 0, 0),),
        )
    );
}

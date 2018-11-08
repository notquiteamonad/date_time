[DateTime][docsrs]: High-Level Date and Time for Rust
=====================================================

[![DateTime on Travis CI][travis-image]][travis]
[![Chrono on crates.io][cratesio-image]][cratesio]
[![DateTime on docs.rs][docsrs-image]][docsrs]

[travis-image]: https://travis-ci.com/samueldple/date_time.svg?branch=master
[travis]: https://travis-ci.com/samueldple/date_time
[cratesio-image]: https://img.shields.io/crates/v/date_time.svg
[cratesio]: https://crates.io/crates/date_time
[docsrs-image]: https://docs.rs/date_time/badge.svg
[docsrs]: https://docs.rs/date_time

Date_Time is a high-level rust library for use in situations where
precision beyond seconds is not necessary.

It handles serialisable dates and times from 01 Jan 0000 at 00:00:00 to
Dec 31 23:59:59.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
date_time = "1.2.0"
```

Then put this in your crate root:

```rust
extern crate date_time;
```

## Overview

All of the types in this library implement `Debug`, `Copy`, and `Clone` traits.

### Times

Times can be generated using the `timetuple::TimeTuple` type.

Times must be instantiated using `TimeTuple::new()` which takes hour, minute, and second parameters. These are then converted into seconds and split apart again to create a tuple between 00:00:00 and 23:59:59.

TimeTuple implements the following traits and is therefore fully comparable with other TimeTuples.

* `PartialOrd`
* `Ord`
* `PartialEq`
* `Eq`

It can also be added to and subtracted from another TimeTuple, but the user must be aware that this will loop around midnight.

For example:

* `TimeTuple::new(22, 0, 0) + TimeTuple::new(1, 0, 0)` will produce `TimeTuple { h: 23, m: 0, s:0 }`
* `TimeTuple::new(22, 0, 0) + TimeTuple::new(3, 0, 0)` will produce `TimeTuple { h: 1, m: 0, s:0 }`

##### Serialisation

`TimeTuple` can be serialised using `to_string()` (generated from Display trait) and `to_hhmm_string()`.

For 8:30:30 AM, the former will produce `"08:30:30"` and the latter will produce `"08:30"`.

A `TimeTuple` can be instantiated by calling `std::str::parse()` with a string in the format of `hh:mm:ss`.

### Dates

Dates can be generated using the `datetuple::DateTuple` and `monthtuple::MonthTuple` types. The `MonthTuple` type is similar to `DateTuple` but doesn't include a day of the month.

#### DateTuple

***NOTE: The month in a `DateTuple` is zero-based.***

`DateTuple` wraps a year, month, and day of month in a struct.

A `DateTuple` can be created using `DateTuple::new()`, passing a year between 0 and 9999 and a month and date which are valid for that year. Feb 29 can be created if the year is a leap year.

`DateTuple` is fully comparable with another `DateTuple` and implements `PartialOrd`, `Ord`, `PartialEq`, and `Eq`.

##### Serialisation

`DateTuple` can be serialised using `to_string()` (generated from Display trait) and `to_readable_string()`.

For 23rd January 2002, the former will produce `"20020023"` and the latter will produce `"23 Jan 2002"`.

A `DateTuple` can be instantiated by calling `std::str::parse()` with a string in the format of `yyyymmdd`.

If listing multiple `DateTuple` objects in a human readable format, you may wish to pad them with a space to the left to ensure alignment. This can be done with the format specifier `{:>11}` in a call such as `format!()`.

#### MonthTuple

***NOTE: The month in a `MonthTuple` is zero-based.***

`MonthTuple` is identical to `DateTuple` but without a day of the month.

It can be instantiated using `MonthTuple::new()`, passing a year between 0000 and 9999 and a month between 0 and 11.

`MonthTuple` is fully comparable with another `MonthTuple` and implements `PartialOrd`, `Ord`, `PartialEq`, and `Eq`.

`MonthTuple` also implements `From<DateTuple>` so a `DateTuple` can be converted to a `MonthTuple` using `MonthTuple::from(date_tuple: DateTuple)`.

##### `next_month` and `previous_month`

`MonthTuple` provides two methods: `next_month` and `previous_month` which consume the `MonthTuple` and return the `MonthTuple` which chronologically follows or precedes it.

These will continue to return the maximum and minimum values of Jan 0000 and Dec 9999 if they are reached.

These methods consume the existing `MonthTuple`.

##### Serialisation

`MonthTuple` can be serialised using `to_string()` (generated from Display trait) and `to_readable_string()`.

For January 2002, the former will produce `"200200"` and the latter will produce `"Jan 2002"`.

A `MonthTuple` can be instantiated by calling `std::str::parse()` with a string in the format of `yyyymm`.

#### The date_utils module

There is an additional module with a single function: `date_utils::is_leap_year(year: u16) -> bool`.

It takes a year and returns whether it is a leap year.

### DateTime

The `date_time_tuple::DateTimeTuple` type wraps a `DateTuple` and a `TimeTuple`.

Like the other modules in this library, it is fully comparable with other `DateTimeTuple` structs.

##### Serialisation

`DateTimeTuple` can be serialised using `to_string()` (generated from Display trait) and `to_readable_string()`.

For 23rd January 2002 at 08:30:30 AM, the former will produce `"20020023@08:30:30"` and the latter will produce `"23 Jan 2002 08:30:30"`.

A `DateTimeTuple` can be instantiated by calling `std::str::parse()` with a string in the format of `yyyymmdd@hh:mm:ss`.

## Limitations

This library was designed for high-level implementations of dates in which precision is not necessary.

For a more precise wrapper of dates, try a crate such as [chrono](https://crates.io/crates/chrono).

* This library is only designed for use when dates need only to be precise to the level of seconds.
* This library is timezone-agnostic; it doesn't deal with any difference between time zones.
* Only datetimes between `01 Jan 0000 00:00:00` and `31 Dec 9999 23:23:23` are supported.

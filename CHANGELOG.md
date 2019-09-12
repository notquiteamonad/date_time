# Changelog

## Version 2.0.1

- Minor documentation improvements

## Version 2.0.0 - _Includes Breaking Changes_

-   **BREAKING CHANGE:** Months in `MonthTuple` and `DateTuple` are now one-based rather than zero-based (i.e. 1 represents January). If you have any serialised data from this library which is to be reingested in code, all months will need to be incremented by 1 before use with version 2.0.0.
-   Added `min_value()` and `max_value()` to `DateTuple`.
-   Added `to_days()` and `from_days()` to `DateTuple`.

## Version 1.5.0

-   Changed `MonthTuple` and `DateTuple`'s serialisation to conform to [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html). Legacy formats can still be parsed via the `FromStr` implementation.

## Version 1.4.4

-   Add `to_minutes` methods for `TimeTuple` and `Duration`
-   Maximised test coverage

## Versions 1.4.2 and 1.4.3

-   Improved documentation

## Version 1.4.1

-   Updated documentation

## Version 1.4.0

-   Added the `Duration` type to hold a time longer than 24 hours.

## Version 1.3.0

-   Added methods to get the next and previous date/month from a `DateTuple`/`MonthTuple`.
-   All of the types now have methods which mutate them by either adding or subtracting discrete values of one of their components.
-   `MonthTuple::this_month()`, `DateTuple::today()`, and `TimeTuple::now()` provide access to values produced from the current time as provided by `std::time::SystemTime::now()`.
-   A `TimeTuple` can now be produced from a number of seconds using `TimeTuple::from_seconds()`.

## Version 1.2.1

-   Added type aliases of `Date`, `Month`, and `Time` for `DateTuple`, `MonthTuple`, and `TimeTuple` respectively.

## Version 1.2.0

-   First public release

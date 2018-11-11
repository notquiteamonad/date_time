Changelog
=========

Version 1.3.0
-------------

* Added methods to get the next and previous date/month from a `DateTuple`/`MonthTuple`.
* All of the types now have methods which mutate them by either adding or subtracting discrete values of one of their components.
* `MonthTuple::this_month()`, `DateTuple::today()`, and `TimeTuple::now()` provide access to values produced from the current time as provided by `std::time::SystemTime::now()`.
* A `TimeTuple` can now be produced from a number of seconds using `TimeTuple::from_seconds()`.

Version 1.2.1
-------------

* Added type aliases of `Date`, `Month`, and `Time` for `DateTuple`, `MonthTuple`, and `TimeTuple` respectively.

Version 1.2.0
-------------

* First public release
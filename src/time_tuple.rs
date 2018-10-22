use std::fmt;

/**
 * TimeTuples should **always** be called with `.resolve()`.
 */
pub struct TimeTuple {
    h: i32,
    m: i32,
    s: i32,
}

impl TimeTuple {
    /**
     * Resolves any overflow/underflow in the TimeTuple.
     */
    pub fn resolve(&self) -> TimeTuple {
        let mut total_seconds = self.s + 60 * self.m + 3600 * self.h;
        while total_seconds > 86400 {
            total_seconds -= 86400;
        }
        while total_seconds < 0 {
            total_seconds += 86400;
        }
        let h = total_seconds / 3600;
        total_seconds -= h * 3600;
        let m = total_seconds / 60;
        total_seconds -= m * 60;
        TimeTuple {
            h,
            m,
            s: total_seconds,
        }
    }

    pub fn to_hhmm_string(&self) -> String {
        format!("{:02}:{:02}", self.h, self.m)
    }
}

impl fmt::Display for TimeTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.h, self.m, self.s)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_no_seconds() {
        let tuple = super::TimeTuple { h: 5, m: 30, s: 0 }.resolve();
        assert_eq!(5, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_no_overlap() {
        let tuple = super::TimeTuple { h: 5, m: 30, s: 30 }.resolve();
        assert_eq!(5, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_second_overlap() {
        let tuple = super::TimeTuple { h: 6, m: 30, s: 90 }.resolve();
        assert_eq!(6, tuple.h);
        assert_eq!(31, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_minute_overlap() {
        let tuple = super::TimeTuple { h: 6, m: 90, s: 30 }.resolve();
        assert_eq!(7, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_hour_overlap() {
        let tuple = super::TimeTuple {
            h: 25,
            m: 30,
            s: 30,
        }.resolve();
        assert_eq!(1, tuple.h);
        assert_eq!(30, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_all_overlap() {
        let tuple = super::TimeTuple {
            h: 25,
            m: 90,
            s: 90,
        }.resolve();
        assert_eq!(2, tuple.h);
        assert_eq!(31, tuple.m);
        assert_eq!(30, tuple.s);
    }

    #[test]
    fn test_minutes_to_hours_overlap() {
        let tuple = super::TimeTuple {
            h: 18,
            m: 420,
            s: 0,
        }.resolve();
        assert_eq!(1, tuple.h);
        assert_eq!(0, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_negative_seconds() {
        let tuple = super::TimeTuple {
            h: 6,
            m: 30,
            s: -60,
        }.resolve();
        assert_eq!(6, tuple.h);
        assert_eq!(29, tuple.m);
        assert_eq!(0, tuple.s);
    }

    #[test]
    fn test_all_negative_overlaps() {
        let tuple = super::TimeTuple {
            h: -3,
            m: -116,
            s: -301,
        }.resolve();
        assert_eq!(18, tuple.h);
        assert_eq!(58, tuple.m);
        assert_eq!(59, tuple.s);
    }

    #[test]
    fn test_to_string() {
        let tuple = super::TimeTuple { h: 3, m: 0, s: 39 }.resolve();
        assert_eq!(String::from("03:00:39"), tuple.to_string())
    }

    #[test]
    fn test_to_hhmm_string() {
        let tuple = super::TimeTuple { h: 3, m: 0, s: 39 }.resolve();
        assert_eq!(String::from("03:00"), tuple.to_hhmm_string())
    }

}

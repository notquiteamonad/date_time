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
        let mut total_seconds = &self.s + 60 * &self.m + 3600 * &self.h;
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
}

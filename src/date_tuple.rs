use date_utils;

#[derive(Debug, Copy, Clone)]
pub struct DateTuple {
    y: u32,
    m: u32,
    d: u32,
}

impl DateTuple {
    pub fn new(y: u32, m: u32, d: u32) -> Result<DateTuple, String> {
        if m <= 11 {
            let max_date = match m {
                1 => {
                    if date_utils::is_leap_year(y) {
                        29
                    } else {
                        28
                    }
                }
                0 => 31,
                2 => 31,
                4 => 31,
                6 => 31,
                7 => 31,
                9 => 31,
                11 => 31,
                _ => 30,
            };
            if d == 0 || d > max_date {
                return Err(format!(
                    "Invalid date in DateTuple: {:?}",
                    DateTuple { y, m, d }
                ));
            }
            Ok(DateTuple { y, m, d })
        } else {
            Err(format!(
                "Invalid month in DateTuple: {:?}\nMonth must be <= 11; Note that months are ZERO-BASED.",
                DateTuple { y, m, d }
            ))
        }
    }

    pub fn get_year(&self) -> u32 {
        self.y
    }

    pub fn get_month(&self) -> u32 {
        self.m
    }

    pub fn get_date(&self) -> u32 {
        self.d
    }
}

#[cfg(test)]
mod tests {

    //todo toString
    //todo equals
    //todo compareTo

    #[test]
    fn test_validity() {
        let valid = [
            super::DateTuple::new(2000, 5, 5),
            super::DateTuple::new(2000, 6, 31),
            super::DateTuple::new(2000, 1, 2),
            super::DateTuple::new(2000, 1, 29),
        ];
        let invalid = [
            super::DateTuple::new(2000, 5, 31),
            super::DateTuple::new(2001, 1, 29),
            super::DateTuple::new(2000, 12, 5),
        ];
        for v in valid.iter() {
            if let Err(_) = v {
                assert!(false);
            }
        }
        for i in invalid.iter() {
            if let Ok(_) = i {
                assert!(false);
            }
        }
    }

}

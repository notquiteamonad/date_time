pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_leap_years() {
        let valid: [u32; 3] = [2000, 2012, 2016];
        let invalid: [u32; 3] = [2100, 2018, 2013];
        for v in valid.iter() {
            assert!(super::is_leap_year(*v));
        }
        for i in invalid.iter() {
            assert!(!super::is_leap_year(*i));
        }
    }
}

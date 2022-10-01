#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::cli::{
        getdata::parse_into_duration,
        validator::{
            validate_date, validate_difficulty, validate_duration, validate_priority, validate_time,
        },
    };

    // DATE TESTS
    #[test]
    fn standard_date() {
        assert_eq!(true, validate_date("21/12/2022"));
    }
    #[test]
    fn date_with_zeros() {
        assert_eq!(true, validate_date("01/01/2000"));
        assert_eq!(true, validate_date("04/11/2000"));
        assert_eq!(true, validate_date("24/03/2000"));
        assert_eq!(false, validate_date("00/00/0000"));
        assert_eq!(false, validate_date("01/00/0000"));
        assert_eq!(false, validate_date("01/02/0000"));
        assert_eq!(false, validate_date("01/02/0001"));
        assert_eq!(true, validate_date("01/02/2021"));
        assert_eq!(true, validate_date("01/12/2021"));
        assert_eq!(true, validate_date("11/12/2021"));
    }
    #[test]
    fn invalid_day() {
        assert_eq!(false, validate_date("32/10/2022"));
        assert_eq!(false, validate_date("43/10/2022"));
    }
    #[test]
    fn invalid_month() {
        assert_eq!(false, validate_date("10/13/2022"));
        assert_eq!(false, validate_date("10/30/2022"));
    }
    #[test]
    fn year_too_early() {
        assert_eq!(false, validate_date("10/11/102"));
        assert_eq!(false, validate_date("10/11/1969"));
    }
    #[test]
    fn day_31_in_month_with_30_days() {
        assert_eq!(false, validate_date("31/02/2022"));
        assert_eq!(false, validate_date("31/04/2022"));
        assert_eq!(false, validate_date("31/06/2022"));
        assert_eq!(false, validate_date("31/09/2022"));
        assert_eq!(false, validate_date("31/11/2022"));
    }
    #[test]
    fn more_than_day_28_in_february_if_not_leap_year() {
        assert_eq!(false, validate_date("29/02/2022"));
        assert_eq!(false, validate_date("29/02/2017"));
        assert_eq!(false, validate_date("29/02/2018"));
        assert_eq!(false, validate_date("29/02/2019"));
        assert_eq!(false, validate_date("29/02/2021"));
        assert_eq!(false, validate_date("30/02/2022"));
        assert_eq!(false, validate_date("31/02/2022"));
        assert_eq!(false, validate_date("32/02/2022"));
    }
    #[test]
    fn day_29_in_february_if_leap_year() {
        assert_eq!(true, validate_date("29/02/2020"));
        assert_eq!(true, validate_date("29/02/2016"));
        assert_eq!(true, validate_date("29/02/2024"));
        assert_eq!(true, validate_date("29/02/2028"));
        assert_eq!(true, validate_date("29/02/2032"));
        assert_eq!(true, validate_date("29/02/2036"));
    }

    // TIME TESTS
    #[test]
    fn standard_time() {
        assert_eq!(true, validate_time("15:30"));
        assert_eq!(true, validate_time("00:30"));
        assert_eq!(true, validate_time("01:30"));
        assert_eq!(true, validate_time("01:00"));
    }
    #[test]
    fn invalid_hour() {
        assert_eq!(true, validate_time("21:30"));
        assert_eq!(false, validate_time("25:30"));
        assert_eq!(false, validate_time("100:30"));
        assert_eq!(false, validate_time("1g:30"));
        assert_eq!(false, validate_time("24:30"));
    }
    #[test]
    fn invalid_minutes() {
        assert_eq!(true, validate_time("22:15"));
        assert_eq!(false, validate_time("22:1f"));
        assert_eq!(false, validate_time("10:70"));
        assert_eq!(false, validate_time("12:000"));
        assert_eq!(false, validate_time("14:64"));
    }

    // DURATION TESTS
    #[test]
    fn duration_format_validation() {
        assert_eq!(true, validate_duration("123d"));
        assert_eq!(true, validate_duration("123 d"));
        assert_eq!(true, validate_duration("12min"));
        assert_eq!(true, validate_duration("12 min"));
        assert_eq!(true, validate_duration("12m"));
        assert_eq!(true, validate_duration("12 m"));
        assert_eq!(true, validate_duration("12h"));
        assert_eq!(true, validate_duration("12 h"));
        assert_eq!(false, validate_duration("-123d"));
        assert_eq!(false, validate_duration("-123 d"));
        assert_eq!(false, validate_duration("-123 min"));
        assert_eq!(false, validate_duration("-123min"));
        assert_eq!(false, validate_duration("-123 h"));
        assert_eq!(false, validate_duration("-123h"));
    }

    // DIFFICULTY TESTS
    #[test]
    fn invalid_difficulty_format() {
        assert_eq!(true, validate_difficulty("2"));
        assert_eq!(true, validate_difficulty("8"));
        assert_eq!(true, validate_difficulty("0"));
        assert_eq!(false, validate_difficulty("11"));
        assert_eq!(false, validate_difficulty("-1"));
        assert_eq!(false, validate_difficulty("-10"));
        assert_eq!(false, validate_difficulty("d10"));
        assert_eq!(false, validate_difficulty("1g0"));
        assert_eq!(false, validate_difficulty(""));
    }
    // PRIORITY TESTS
    #[test]
    fn invalid_priority_format() {
        assert_eq!(true, validate_priority("2"));
        assert_eq!(true, validate_priority("0"));
        assert_eq!(false, validate_priority("-1"));
        assert_eq!(false, validate_priority("-10"));
        assert_eq!(false, validate_priority("d10"));
        assert_eq!(false, validate_priority("1g0"));
        assert_eq!(false, validate_priority(""));
    }
    #[test]
    fn parse_duration() {
        assert_eq!(Duration::minutes(15), parse_into_duration("15m"));
        assert_eq!(Duration::minutes(15), parse_into_duration("15min"));
        assert_eq!(Duration::minutes(15), parse_into_duration("15 m"));
        assert_eq!(Duration::minutes(15), parse_into_duration("15 min"));
        assert_eq!(Duration::hours(4), parse_into_duration("4h"));
        assert_eq!(Duration::hours(4), parse_into_duration("4 h"));
        assert_eq!(Duration::days(2), parse_into_duration("2d"));
        assert_eq!(Duration::days(2), parse_into_duration("2 d"));
    }
}

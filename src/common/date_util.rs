use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, TimeDelta};
use chrono_tz::Tz;

pub const DEFAULT_DATE_STR: &str = "%Y-%m-%d";

pub const DEFAULT_DATE_TIME_STR: &str = "%Y-%m-%d %H:%M:%S";

pub fn to_date(s: &str) -> NaiveDate {
    return to_date_with_pattern(s, DEFAULT_DATE_STR);
}

pub fn to_date_with_pattern(s: &str, pattern: &str) -> NaiveDate {
    return NaiveDate::parse_from_str(s, pattern).unwrap();
}

pub fn to_datetime(s: &str) -> NaiveDateTime {
    return to_datetime_with_pattern(s, DEFAULT_DATE_TIME_STR);
}

pub fn to_datetime_with_pattern(s: &str, pattern: &str) -> NaiveDateTime {
    return NaiveDateTime::parse_from_str(s, pattern).unwrap();
}

pub fn naive_date_offset_days(date: NaiveDate, offset: i64) -> NaiveDate {
    naive_date_offset(date, Duration::days(offset))
}

pub fn naive_date_offset(date: NaiveDate, rhs: TimeDelta) -> NaiveDate {
    date.checked_add_signed(rhs).unwrap()
}

pub fn naive_date_time_offset_days(date_time: NaiveDateTime, offset: i64) -> NaiveDateTime {
    naive_date_time_offset(date_time, Duration::days(offset))
}

pub fn naive_date_time_offset(date_time: NaiveDateTime, rhs: TimeDelta) -> NaiveDateTime {
    date_time.checked_add_signed(rhs).unwrap()
}

pub fn date_time_offset_days(date_time: DateTime<Tz>, offset: i64) -> DateTime<Tz> {
    date_time_offset(date_time, Duration::days(offset))
}

pub fn date_time_offset(date_time: DateTime<Tz>, rhs: TimeDelta) -> DateTime<Tz> {
    date_time.checked_add_signed(rhs).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn it_works() {
        let date_str = "2024-06-22";
        let date = to_date(date_str);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 22);
    }

    #[test]
    fn test_dat_offset() {
        let d = to_date("2024-06-22");
        let d1 = naive_date_offset_days(d, -2);
        assert_eq!(d1.year(), 2024);
        assert_eq!(d1.month(), 6);
        assert_eq!(d1.day(), 20);
    }
}

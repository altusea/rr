use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime};
use chrono_tz::Tz;
use chrono_tz::Tz::Asia__Shanghai;
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

pub fn day_offset(date: NaiveDateTime, offset: i64) -> NaiveDateTime {
    date.checked_add_signed(Duration::days(offset)).unwrap()
}

pub fn days_offset(date_time: DateTime<Tz>, offset: i64) -> DateTime<Tz> {
    date_time
        .checked_add_signed(Duration::days(offset))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use super::*;

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
        let d1 = day_offset(d, -2);
        assert_eq!(d1.year(), 2024);
        assert_eq!(d1.month(), 6);
        assert_eq!(d1.day(), 20);
    }
}

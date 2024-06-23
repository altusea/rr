use chrono::{DateTime, Local, NaiveDate, TimeZone};
use chrono_tz::Tz::Asia__Shanghai;
use rr::common::date_util::days_offset;
use std::fmt::Debug;

fn main() {
    let dt1: DateTime<Local> = Local::now();
    println!("{}", dt1);
    let dt2: DateTime<Local> = Local.timestamp_opt(0, 0).unwrap();
    println!("{}", dt2);

    let a: DateTime<chrono_tz::Tz> = Asia__Shanghai
        .with_ymd_and_hms(2024, 6, 22, 16, 22, 25)
        .unwrap();
    println!("{}", a);

    let b = days_offset(a, 10);
    println!("{}", b)
}

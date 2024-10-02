use chrono::{Datelike, NaiveDate};
pub use chrono::Weekday as wd;

fn is_leap_year(year: i32) -> bool {year % 4 == 0 && year % 100 != 0 || year % 400 == 0}

pub fn middle_day(year: i32) -> Option<wd> {
    if is_leap_year(year) {return None;}
    Some(NaiveDate::from_yo_opt(year, 183)?.weekday())
}

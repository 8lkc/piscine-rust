use middle_day::middle_day;
use chrono::Weekday as wd;

fn main() {
    assert_eq!(wd::Mon, middle_day(2019).unwrap());
}

use chrono::{Datelike, NaiveDate, Duration, Local};

pub fn days_left(deadline: NaiveDate) -> i64 {
    let today = Local::now().date_naive();
    let mut current_date = today;
    let mut weekdays = 0;

    while current_date <= deadline {
        if current_date.weekday().num_days_from_monday() < 5 {
            weekdays += 1;
        }
        current_date += Duration::days(1);
    }
    weekdays
}

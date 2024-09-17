use chrono::{Datelike, Duration, Local, NaiveDate};

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

pub fn min_hours_per_day(
    days_left: i64,
    hours_invested: Option<f64>,
    hours_needed: Option<f64>,
) -> f64 {
    (hours_needed.unwrap_or(0.0) - hours_invested.unwrap_or(0.0)) / days_left as f64
}

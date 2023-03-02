use chrono::{Local, NaiveDate};
use regex::Regex;

pub struct DateOption {
    pub date: NaiveDate,
    pub days_from_now: i64,
}

impl DateOption {
    pub fn new(date_str: &str) -> Self {
        let parsed_date = match date_str {
            "date=now" => Local::now().date_naive(),
            date_str => match cut_date(date_str) {
                Some(date_str) => match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                    Ok(date_time) => date_time,
                    Err(_) => Local::now().date_naive(),
                },
                None => Local::now().date_naive(),
            },
        };
        let diff = parsed_date.signed_duration_since(Local::now().date_naive());
        let days_from_now = diff.num_days();
        Self {
            date: parsed_date,
            days_from_now,
        }
    }
}

fn cut_date(date_str: &str) -> Option<String> {
    let pattern = match Regex::new(r"date=(\d{4}-\d{2}-\d{2})") {
        Ok(pattern) => Some(pattern),
        Err(_) => None,
    };

    match pattern {
        Some(pattern) => pattern
            .captures(date_str)
            .map(|captures| captures[1].to_string()),
        None => None,
    }
}

impl ToString for DateOption {
    fn to_string(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate};

    use crate::utils::date::{DateOption, Local};
    #[test]
    fn date_for_now() {
        let curr_date = Local::now().date_naive();
        let date_param = date_option_string(curr_date);
        let date_option = DateOption::new(&date_param);
        assert_date_option(date_option, curr_date, 0)
    }

    #[test]
    fn date_for_72_days_ago() {
        let days_ago = -72;
        let date = date_from_now(days_ago);
        let date_param = date_option_string(date);
        assert_date_option(DateOption::new(&date_param), date, days_ago);
    }

    #[test]
    fn date_for_86_days_in_future() {
        let days_in_future = 86;
        let date = date_from_now(days_in_future);
        let date_param = date_option_string(date);
        assert_date_option(DateOption::new(&date_param), date, days_in_future);
    }

    #[test]
    fn date_for_7_days_in_future() {
        let days_in_future = 7;
        let date = date_from_now(days_in_future);
        let date_param = date_option_string(date);
        assert_date_option(DateOption::new(&date_param), date, days_in_future);
    }

    fn date_option_string(date: NaiveDate) -> String {
        let date_string = date.to_string();
        format!("date={date_string}")
    }

    fn date_from_now(days: i64) -> NaiveDate {
        Local::now().date_naive() + Duration::days(days)
    }

    fn assert_date_option(given_date_option: DateOption, date: NaiveDate, days_from_now: i64) {
        assert_eq!(given_date_option.date, date);
        assert_eq!(given_date_option.days_from_now, days_from_now);
    }
}

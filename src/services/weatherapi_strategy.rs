use crate::services::weather_service::{ForecastResponseData, ForecastStrategy};
use crate::utils::date::DateOption;

use serde::{Deserialize, Serialize};

/*
In case we need to support date select for weatherapi.com service we need to use 2 separate requests:
from 1 Jan 2010 till now:
https://api.weatherapi.com/v1/history.json?key=<api_key>0>&q=Vyshgorod, Ukraine&dt=2023-03-01
Date on or after 1st Jan, 2010 in yyyy-MM-dd format

from now to now + 14 days:
https://api.weatherapi.com/v1/forecast.json?key=<api_key>&q=Vyshgorod, Ukraine&days=12&aqi=no&alerts=no

for future > 14 days from now:
https://api.weatherapi.com/v1/future.json?key=<api_key>&q=Vyshgorod, Ukraine&dt=2023-03-15

 */
pub struct WeatherApiStrategy;

const API_KEY: &str = env!("WEATHERAPI_API_KEY");
const URL: &str = "https://api.weatherapi.com/v1/";

impl ForecastStrategy for WeatherApiStrategy {
    fn build_request(&self, address: &str, date: &str) -> Result<String, &'static str> {
        let date = DateOption::new(date);
        match WeatherRequest::new(address, date).query(API_KEY) {
            Ok(query) => Ok(query),
            Err(_) => Err("Can't build query"),
        }
    }

    fn build_response(
        &self,
        request_result: reqwest::blocking::Response,
    ) -> Result<ForecastResponseData, &'static str> {
        match request_result.json::<WeatherApiResponse>() {
            Ok(data) => build_forecast(data),
            Err(_) => Err("Incorrect data from weather service: can't parse"),
        }
    }
}

fn build_forecast(data: WeatherApiResponse) -> Result<ForecastResponseData, &'static str> {
    let location = data.location;
    let forecastday = match data.forecast.forecastday.last() {
        Some(forecastday) => forecastday,
        None => return Err("Error: No forecast for this day"),
    };
    let day = &forecastday.day;

    Ok(ForecastResponseData {
        location: format!(
            "{}, {}, {}",
            location.name, location.region, location.country
        ),
        date: forecastday.date.clone(),
        max_temp: day.maxtemp_c,
        min_temp: day.mintemp_c,
        avg_temp: day.avgtemp_c,
        maxwind_kph: day.maxwind_kph,
        avghumidity: day.avghumidity,
        condition: day.condition.text.clone(),
    })
}

struct WeatherRequest {
    address: String,
    date_option: DateOption,
}

impl WeatherRequest {
    fn new(address: &str, date_option: DateOption) -> Self {
        Self {
            address: address.to_string(),
            date_option,
        }
    }

    fn query(&self, api_key: &str) -> Result<String, url::ParseError> {
        let mut url = url::Url::parse(URL)?.join(&self.forecast_method())?;
        url.set_query(Some(&self.set_date_option()));
        url.query_pairs_mut()
            .append_pair("key", api_key)
            .append_pair("q", &self.address);

        Ok(url.to_string())
    }

    fn forecast_method(&self) -> String {
        let method = if self.date_option.days_from_now < 0 {
            "history.json"
        } else if (0..=13).contains(&self.date_option.days_from_now) {
            "forecast.json"
        } else {
            "future.json"
        };
        method.to_string()
    }

    fn set_date_option(&self) -> String {
        match self.forecast_method().as_str() {
            "forecast.json" => {
                "days=".to_string() + &(self.date_option.days_from_now + 1).to_string()
            }
            _ => "dt=".to_string() + &self.date_option.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherApiResponse {
    location: WeatherLocation,
    forecast: WeatherForecastDays,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherLocation {
    name: String,
    region: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherForecastDays {
    forecastday: Vec<WeatherForecastDay>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherForecastDay {
    date: String,
    day: Day,
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    maxtemp_c: f32,
    mintemp_c: f32,
    avgtemp_c: f32,
    maxwind_kph: f32,
    avghumidity: f32,
    condition: Condition,
}

#[derive(Serialize, Deserialize, Debug)]
struct Condition {
    text: String,
}

#[cfg(test)]
mod tests {
    use super::WeatherRequest;
    use crate::utils::date::DateOption;
    use chrono::{Duration, Local, NaiveDate};

    #[test]
    fn weather_request_for_17_days_in_past() {
        let date = date_from_now(-17);
        let query = query_for_date(date);
        let sample = "https://api.weatherapi.com/v1/history.json?".to_string()
            + "dt="
            + &date.to_string()
            + "&key=some_api_key&q=test_city%2C+UA";
        assert_eq!(query, sample);
    }

    #[test]
    fn weather_request_for_now() {
        let date = date_from_now(0);
        let query = query_for_date(date);
        let sample = "https://api.weatherapi.com/v1/forecast.json?".to_string()
            + "days=1"
            + "&key=some_api_key&q=test_city%2C+UA";
        assert_eq!(query, sample);
    }

    #[test]
    fn weather_request_for_13_days_future() {
        let date = date_from_now(13);
        let query = query_for_date(date);
        let sample = "https://api.weatherapi.com/v1/forecast.json?".to_string()
            + "days=14"
            + "&key=some_api_key&q=test_city%2C+UA";
        assert_eq!(query, sample);
    }

    #[test]
    fn weather_request_for_14_days_future() {
        let date = date_from_now(14);
        let query = query_for_date(date);
        let sample = "https://api.weatherapi.com/v1/future.json?".to_string()
            + "dt="
            + &date.to_string()
            + "&key=some_api_key&q=test_city%2C+UA";
        assert_eq!(query, sample);
    }

    fn date_option_string(date: NaiveDate) -> String {
        let date_string = date.to_string();
        format!("date={date_string}")
    }

    fn date_from_now(days: i64) -> NaiveDate {
        Local::now().date_naive() + Duration::days(days)
    }

    fn query_for_date(date: NaiveDate) -> String {
        let date_param = date_option_string(date);
        let date_option = DateOption::new(&date_param);
        WeatherRequest::new("test_city, UA", date_option)
            .query("some_api_key")
            .unwrap()
    }
}

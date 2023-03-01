use crate::services::weather_service::{ForecastResponseData, ForecastStrategy};
use crate::utils::date::DateOption;

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
        println!("{:?}", &date.date);
        println!("{:?}", &date.days_from_now);
        let query = URL.to_string() + "/forecast.json?key=" + API_KEY + "q=" + address;
        println!("WeatherApi query: {query}");
        Ok(query)
    }

    fn build_response(&self, request_result: &str) -> Result<ForecastResponseData, &'static str> {
        println!("Results for WeatherApi: {request_result}");
        Ok(ForecastResponseData {
            data: "ok".to_string(),
        })
    }
}

use crate::services::weather_service::{ForecastResponseData, ForecastStrategy};
pub struct OpenWeatherStrategy;

// const API_KEY &str = env!("OPENWEATHER_API_KEY");

impl ForecastStrategy for OpenWeatherStrategy {
    fn build_request(&self, address: &str, date: &str) -> Result<String, &'static str> {
        println!("Results for OpenWeather {address} {date}");
        Ok("ok".to_string())
    }

    fn build_response(
        &self,
        request_result: reqwest::blocking::Response,
    ) -> Result<ForecastResponseData, &'static str> {
        println!("Results for OpenWeather {request_result:?}");
        Err("error")
    }
}

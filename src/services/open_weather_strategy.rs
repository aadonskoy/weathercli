use crate::services::weather_service::{ForecastStrategy, ForecastResponseData };
pub struct OpenWeatherStrategy;

impl ForecastStrategy for OpenWeatherStrategy {
    fn build_request(&self, address: &str, date: &str) -> Result<&str, &'static str> {
        let openweather_api_key: &'static str = env!("OPENWEATHER_API_KEY");
        println!("Results for OpenWeather");
        Ok("ok")
    }

    fn build_response(&self, request_result: &str) -> Result<ForecastResponseData, &'static str> {
        println!("Results for OpenWeather");
        Ok(ForecastResponseData { data: "ok".to_string() })
    }
}
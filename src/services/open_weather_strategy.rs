use crate::services::weather_service::{ForecastStrategy, ForecastResponseData };
pub struct OpenWeatherStrategy;

impl ForecastStrategy for OpenWeatherStrategy {
    fn build_request(&self, address: String, date: String) -> Result<String, &'static str> {
        let openweather_api_key: &'static str = env!("OPENWEATHER_API_KEY");
        println!("Results for OpenWeather");
        Ok("ok".to_string())
    }

    fn build_response(&self, request_result: String) -> Result<ForecastResponseData, &'static str> {
        println!("Results for OpenWeather");
        Ok(ForecastResponseData { data: "ok".to_string() })
    }
}
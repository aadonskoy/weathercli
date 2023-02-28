use crate::utils::config;
use crate::services::{
    open_weather_strategy::OpenWeatherStrategy,
    weatherapi_strategy::WeatherApiStrategy
};

pub enum WeatherService {
    OpenWeather,
    WeatherApi,
    AccuWeather,
    AerisWeather,
    UnknownService,
    MissingService,
}

fn render_forecast_data(response_data: Result<ForecastResponseData, &'static str>) {
    match response_data {
        Ok(response_data) => response_data.render(),
        Err(error) => println!("Error: {error}")
    };
}

pub fn weather_forecast(address: String, date: String) {
    let response_data = match config::get_provider() {
        WeatherService::OpenWeather => WeatherForecastData::new(OpenWeatherStrategy).get_forecast(address, date),
        WeatherService::WeatherApi => WeatherForecastData::new(WeatherApiStrategy).get_forecast(address, date),
        _ => WeatherForecastData::new(WeatherApiStrategy).get_forecast(address, date),
    };

    render_forecast_data(response_data);
}

pub trait ForecastStrategy {
    fn build_request(&self, address: String, date: String) -> Result<String, &'static str>;

    fn build_response(&self, request_result: String) -> Result<ForecastResponseData, &'static str>;
}

struct WeatherForecastData<T: ForecastStrategy> {
    forecast_strategy: T,
}

impl<T: ForecastStrategy> WeatherForecastData<T> {
    fn new(forecast_strategy: T) -> Self {
        Self { forecast_strategy }
    }

    fn get_forecast(&self, address: String, date: String) -> Result<ForecastResponseData, &'static str> {
        let query = self.forecast_strategy.build_request(address, date).unwrap();
        let results = self.request(query).unwrap();
        self.forecast_strategy.build_response(results)
    }

    fn request(&self, query: String) -> Result<String, &'static str> {
        Ok("some response".to_string())
    }
}
pub struct ForecastResponseData {
    pub data: String,
}

impl ForecastResponseData {
    fn render(&self) {
        println!("RES: {:?}", self.data);
    }
}

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

pub fn weather_forecast(address: &str, date: &str) {
    let response_data = match config::get_provider() {
        WeatherService::OpenWeather => WeatherForecastData::new(OpenWeatherStrategy).get_forecast(address, date),
        WeatherService::WeatherApi => WeatherForecastData::new(WeatherApiStrategy).get_forecast(address, date),
        _ => WeatherForecastData::new(WeatherApiStrategy).get_forecast(address, date),
    };

    render_forecast_data(response_data);
}

pub trait ForecastStrategy {
    fn build_request(&self, address: &str, date: &str) -> Result<&str, &'static str>;

    fn build_response(&self, request_result: &str) -> Result<ForecastResponseData, &'static str>;
}

struct WeatherForecastData<T: ForecastStrategy> {
    forecast_strategy: T,
}

impl<T: ForecastStrategy> WeatherForecastData<T> {
    fn new(forecast_strategy: T) -> Self {
        Self { forecast_strategy }
    }

    fn get_forecast(&self, address: &str, date: &str) -> Result<ForecastResponseData, &'static str> {
        let results = match self.forecast_strategy.build_request(address, date) {
            Ok(query) => match self.request(query) {
                Ok(result) => result,
                Err(err) => return Err(err)
            },
            Err(error) => return Err(error)
        };
        self.forecast_strategy.build_response(results)
    }

    fn request(&self, query: &str) -> Result<&str, &'static str> {
        Ok("some response")
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

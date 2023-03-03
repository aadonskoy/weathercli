use crate::services::{
    open_weather_strategy::OpenWeatherStrategy, weatherapi_strategy::WeatherApiStrategy,
};
use crate::utils::config;

pub enum WeatherService {
    OpenWeather,
    WeatherApi,
    // AccuWeather,
    // AerisWeather,
    UnknownService,
    MissingService,
}

fn render_forecast_data(response_data: Result<ForecastResponseData, &'static str>) {
    match response_data {
        Ok(response_data) => response_data.render(),
        Err(error) => println!("Error: {error}"),
    };
}

pub fn weather_forecast(address: &str, date: &str) {
    let response_data = match config::get_provider() {
        WeatherService::OpenWeather => {
            WeatherForecastData::new(OpenWeatherStrategy).get_forecast(address, date)
        }
        WeatherService::WeatherApi => {
            WeatherForecastData::new(WeatherApiStrategy).get_forecast(address, date)
        }
        _ => WeatherForecastData::new(WeatherApiStrategy).get_forecast(address, date),
    };

    render_forecast_data(response_data);
}

pub trait ForecastStrategy {
    fn build_request(&self, address: &str, date: &str) -> Result<String, &'static str>;

    fn build_response(
        &self,
        response: reqwest::blocking::Response,
    ) -> Result<ForecastResponseData, &'static str>;
}

struct WeatherForecastData<T: ForecastStrategy> {
    forecast_strategy: T,
}

impl<T: ForecastStrategy> WeatherForecastData<T> {
    fn new(forecast_strategy: T) -> Self {
        Self { forecast_strategy }
    }

    fn get_forecast(
        &self,
        address: &str,
        date: &str,
    ) -> Result<ForecastResponseData, &'static str> {
        let results = match self.forecast_strategy.build_request(address, date) {
            Ok(query) => match reqwest::blocking::get(query) {
                Ok(response) => self.forecast_strategy.build_response(response),
                Err(_) => Err("Can't retrieve weather data"),
            },
            Err(_) => Err("Can't create request"),
        };
        results
    }
}

pub struct ForecastResponseData {
    pub location: String,
    pub date: String,
    pub max_temp: f32,
    pub min_temp: f32,
    pub avg_temp: f32,
    pub maxwind_kph: f32,
    pub avghumidity: f32,
    pub condition: String,
}

impl ForecastResponseData {
    fn render(&self) {
        println!(
            "== Weather forecast for {} on date {} ==",
            self.location, self.date
        );
        if !self.condition.is_empty() {
            let cond_emoji = match self.condition.to_lowercase().as_str() {
                "sunny" => "☀️",
                "overcast" => "🌥️",
                "blizzard" | "snow" => "🌨️",
                "patchy rain possible" => "🌦️",
                "clouds" => "☁️",
                _ => "",
            };
            println!("{} {}", self.condition, cond_emoji);
        }
        println!(
            "🌡️  Average temperature {}°C (Min {}°C and Max {}°C)",
            self.avg_temp, self.min_temp, self.max_temp
        );
        println!("💧  Average humidity {}", self.avghumidity);
        println!("💨  Wind max speed {} Km/h", self.maxwind_kph);
    }
}

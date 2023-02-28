use serde::{Serialize, Deserialize};
use crate::services::weather_service::{WeatherService};

pub fn set_provider(provider: &Option<String>) {
    let hint = "Please use: openweather, weatherapi, accuweather or aerisweather.";

    match WeatherService::from(provider.to_owned()) {
        WeatherService::MissingService => println!("No service provided. {hint}"),
        WeatherService::UnknownService => println!("Service unknown. {hint}"),
        permitted_provider => {
            write_config(permitted_provider);
        }
    }
}

pub fn get_provider() -> WeatherService {
    let config: Result<WeatherCliConfig, confy::ConfyError> = confy::load("weather-cli", None);
    match config {
        Ok(config) => WeatherService::from(Some(config.provider)),
        Err(_) => WeatherService::OpenWeather
    }
}

#[derive(Serialize, Deserialize)]
struct WeatherCliConfig{
    provider: String,
}

impl From<Option<String>> for WeatherService {
    fn from(provider: Option<String>) -> WeatherService {
        match provider {
            Some(provider) => match provider.to_lowercase().as_str() {
                "openweather" => WeatherService::OpenWeather,
                "weatherapi" => WeatherService::WeatherApi,
                "accuweather" => WeatherService::AccuWeather,
                "aerisweather" => WeatherService::AerisWeather,
                _ => WeatherService::UnknownService,
            },
            None => WeatherService::MissingService,
        }
    }
}

impl Default for WeatherCliConfig {
    fn default() -> Self {
        Self {
            provider: "openweather".to_string(),
        }
    }
}

fn maybe_provider_to_string(provider: &Option<WeatherService>) -> Option<String> {
    match provider {
        Some(WeatherService::OpenWeather) => Some("openweather".to_string()),
        Some(WeatherService::WeatherApi) => Some("weatherapi".to_string()),
        Some(WeatherService::AccuWeather) => Some("accuweather".to_string()),
        Some(WeatherService::AerisWeather) => Some("aerisweather".to_string()),
        Some(_) => None,
        None => None,
    }
}

fn write_config(provider: WeatherService) {
    match maybe_provider_to_string(&Some(provider)) {
        Some(provider_str) => {
            match confy::store("weather-cli", None, WeatherCliConfig { provider: provider_str }) {
                Err(err) => print!("Can't save config: {err}"),
                Ok(_) => println!("Config updated")
            }
        }
        None => println!("not write"),
    }
}
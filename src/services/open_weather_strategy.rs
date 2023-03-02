use crate::services::weather_service::{ForecastResponseData, ForecastStrategy};
use crate::utils::date::DateOption;
use serde::{Deserialize, Serialize};
pub struct OpenWeatherStrategy;

/*
For this provider for free we can use only weather forecast current and 16 days in future:
Now and up to 16 days in future
https://api.openweathermap.org/data/2.5/forecast/daily?q=Kyiv,%20UA&cnt=16&units=metric&appid=<app_id>

Historical data isn't available as well.
*/

const API_KEY: &str = env!("OPENWEATHER_API_KEY");
const URL: &str = "https://api.openweathermap.org/data/2.5/";

impl ForecastStrategy for OpenWeatherStrategy {
    fn build_request(&self, address: &str, date: &str) -> Result<String, &'static str> {
        let date = DateOption::new(date);
        match WeatherRequest::new(address, date).query(API_KEY) {
            Ok(query) => Ok(query),
            Err(error) => {
                println!("{error}");
                Err(error)
            }
        }
    }

    fn build_response(
        &self,
        request_result: reqwest::blocking::Response,
    ) -> Result<ForecastResponseData, &'static str> {
        match request_result.json::<WeatherApiResponse>() {
            Ok(data) => Ok(build_forecast(data)),
            Err(_) => Err("Incorrect data from weather service: can't parse"),
        }
    }
}

fn build_forecast(data: WeatherApiResponse) -> ForecastResponseData {
    let weatherday = match data.list.last() {
        Some(weatherday) => weatherday,
        None => {
            println!("Error: No forecast for this day");
            panic!("Exit...");
        }
    };
    let date =
        chrono::NaiveDateTime::from_timestamp_opt(weatherday.dt, 0).expect("error decoding date");
    println!("DATE: {}", weatherday.dt);
    let date_string = date.format("%Y-%m-%d").to_string();

    ForecastResponseData {
        location: format!("{}, {}", data.city.name, data.city.country),
        date: date_string,
        max_temp: weatherday.temp.max,
        min_temp: weatherday.temp.min,
        avg_temp: weatherday.temp.day,
        maxwind_kph: weatherday.speed,
        avghumidity: weatherday.humidity,
        condition: match weatherday.weather.first() {
            Some(weather) => weather.main.clone(),
            None => "".to_string(),
        },
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherApiResponse {
    city: City,
    list: Vec<Day>,
}

#[derive(Serialize, Deserialize, Debug)]
struct City {
    name: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    dt: i64,
    temp: Temp,
    humidity: f32,
    weather: Vec<DayWeather>,
    speed: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temp {
    day: f32,
    min: f32,
    max: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct DayWeather {
    main: String,
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

    fn query(&self, api_key: &str) -> Result<String, &'static str> {
        let date_option = "&cnt=".to_string() + &(&self.date_option.days_from_now + 1).to_string();

        if self.is_date_available() {
            Ok(URL.to_string()
                + "forecast/daily"
                + "?q="
                + &self.address.replace(' ', "%20")
                + &date_option
                + "&units=metric"
                + "&appid="
                + api_key)
        } else {
            Err("Sorry, selected service doesn't support date in past or date in future more than 16 days from now")
        }
    }

    fn is_date_available(&self) -> bool {
        (1..=17).contains(&(self.date_option.days_from_now + 1))
    }
}

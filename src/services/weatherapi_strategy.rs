use crate::services::weather_service::{ForecastStrategy, ForecastResponseData };

/*
    In case we need to support date select for weatherapi.com service we need to use 2 separate requests:
    from 1 Jan 2010 till now:
    https://api.weatherapi.com/v1/history.json?key=<api_key>0>&q=Vyshgorod, Ukraine&dt=2023-03-01
    Date on or after 1st Jan, 2010 in yyyy-MM-dd format

    from now to now + 14 days:
    https://api.weatherapi.com/v1/forecast.json?key=<api_key>&q=Vyshgorod, Ukraine&days=12&aqi=no&alerts=no

    for future > 14 days from now:
    https://api.weatherapi.com/v1/future.json?key=584a4ba3026d45aa8c2112100232702&q=Vyshgorod, Ukraine&dt=2023-03-15

 */
pub struct WeatherApiStrategy;

impl ForecastStrategy for WeatherApiStrategy {
    fn build_request(&self, address: String, date: String) -> Result<String, &'static str> {
        let weatherapi_api_key: &'static str = env!("WEATHERAPI_API_KEY");

        println!("Results for WeatherApi");
        Ok("ok".to_string())
    }

    fn build_response(&self, request_result: String) ->  Result<ForecastResponseData, &'static str> {
        println!("Results for WeatherApi");
        Ok(ForecastResponseData { data: "ok".to_string() })
    }
}

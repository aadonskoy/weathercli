# WeatherCLI
Weather forecast cli utility with ability to select weather provider service.

Help
```
weather help
```

Get weather forecast for current date:
```
weather get 'Kyiv, UA'
```

or for other date:
```
weather get 'Kyiv, UA' date=2023-03-25
```

Configure weather provider (OpenWeather by default)
```
weather config <provider_service>
```
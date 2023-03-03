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
weather get 'Vyshgorod, UA' date=2023-03-04
```

<img width="761" alt="weathercli-upd" src="https://user-images.githubusercontent.com/1927898/222672552-0c5599c1-7348-4703-aab4-b87fea35f485.png">


Configure weather provider (OpenWeather by default)
```
weather config <provider_service>
```

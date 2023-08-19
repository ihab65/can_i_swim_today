use structopt::StructOpt;
use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(StructOpt, Debug)]
struct Cli {
    city: String,
    country_code: String
}
#[derive(Debug, Deserialize, Serialize)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32
}
#[derive(Debug, Deserialize, Serialize)]
struct Coord {
    lon: f64,
    lat: f64
}
#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    details: Details
}
#[derive(Debug, Deserialize, Serialize)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String
}
#[derive(Debug, Deserialize, Serialize)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64,
    humidity: i32,
}
#[derive(Debug, Deserialize, Serialize)]
struct Wind {
    speed: f64,
    deg: f64,
    gust: f64
}
#[derive(Debug, Deserialize, Serialize)]
struct Clouds {
    all: i32
}
#[derive(Debug, Deserialize, Serialize)]
struct Sys {
    country: String,
    sunrise: i32,
    sunset: i32,
}

impl Forecast {
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid=5e205416dfa82a16871aa55675e6aab6", city, country_code);
        let url = Url::parse(&*url)?;
        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;

        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    let direction = match response.wind.deg {
        deg if deg < 11.25 => "North",
        deg if deg < 33.75 => "North-Northeast",
        deg if deg < 56.25 => "Northeast",
        deg if deg < 78.75 => "East-Northeast",
        deg if deg < 101.25 => "East",
        deg if deg < 123.75 => "East-Southeast",
        deg if deg < 146.25 => "Southeast",
        deg if deg < 168.75 => "South-Southeast",
        deg if deg < 191.25 => "South",
        deg if deg < 213.75 => "South-Southwest",
        deg if deg < 236.25 => "Southwest",
        deg if deg < 258.75 => "West-Southwest",
        deg if deg < 281.25 => "West",
        deg if deg < 303.75 => "West-Northwest",
        deg if deg < 326.25 => "Northwest",
        deg if deg < 348.75 => "North-Northwest",
        deg if deg > 348.75 => "North",
        deg if deg > 360.00 => "this must be an error",
        _ => "Invalid input",
    };

    println!("our city: {}, our country: {}, temp: {}", args.city, args.country_code, response.main.feels_like);
    println!("wind speed: {} km/h, wind_derction: {}", (response.wind.speed * 3.6).round(), direction);
    Ok(())
}

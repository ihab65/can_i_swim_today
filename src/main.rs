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
    humidity: f64,
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

fn get_wind_direction(wind_deg: f64) -> &'static str {
    let direction = match wind_deg {
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
    direction
}

fn comfort_level(humidity: f64) -> &'static str {
    match humidity {
        level if level <= 20.0 => "Very low humidity, dry conditions",
        level if level <= 40.0 => "Low humidity, generally comfortable",
        level if level <= 60.0 => "Moderate humidity, pleasant",
        level if level <= 75.0 => "High humidity, may feel a bit sticky",
        level if level <= 90.0 => "Very high humidity, uncomfortable",
        _ => "Extremely high humidity, oppressive",
    }
}

fn swimming_conditions(wind_speed: f64) -> &'static str {
    match wind_speed {
        speed if speed <= 10.0 => "Calm conditions, minimal impact on swimming",
        speed if speed <= 20.0 => "Gentle breeze, slight ripples, suitable for swimming",
        speed if speed <= 30.0 => "Moderate breeze, small waves may form",
        speed if speed <= 40.0 => "Increasing breeze, potential for slightly choppy water",
        speed if speed <= 50.0 => "Moderate winds, water choppy, swimming more challenging",
        _ => "High winds, rough water, not recommended for swimming",
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    let wind_direction = get_wind_direction(response.wind.deg);
    let swimming_conditions = swimming_conditions(response.wind.speed * 3.6);
    let comfort_level = comfort_level(response.main.humidity);
    
    println!("our city: {}, our country: {}, temp: {}", args.city, args.country_code, response.main.temp);
    println!("humidity: {} %, how it feels: {}", response.main.humidity, comfort_level);
    println!("wind speed: {:.2} km/h, wind_derction: {}, swimming_conditions: {}", (response.wind.speed * 3.6), wind_direction, swimming_conditions);
    Ok(())
}

use std::str::pattern::StrSearcher;

use structopt::StructOpt;
use exitfailure::ExitFailure;

#[derive(StructOpt, Debug)]
struct Cli {
    city: String,
    country_code: String
}
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
struct Coord {
    lon: f64,
    lat: f64
}
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String
}
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64,
    humidity: i32,
}
struct Wind {
    speed: f64,
    deg: i32,
}
struct Clouds {
    all: i32
}
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunnset: i32
}

impl Forecast {
    async fn get(city: String, country_code: String) -> Result<Self, ExitFailure> {}
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);
}

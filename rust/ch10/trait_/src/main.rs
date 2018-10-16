extern crate trait_;

use trait_::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%.", self.high_temp, self.low_temp, self.chance_of_precipitation)
    }
    fn author_summary(&self) -> String {
        String::from("news")
    }
}

fn main() {
    let weather = WeatherForecast {
        high_temp: 44.0,
        low_temp: -20.4,
        chance_of_precipitation: 14.3
    };

    println!("{}", weather.summary());
}

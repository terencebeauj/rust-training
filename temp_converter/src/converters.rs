pub fn celsius_to_farenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}

pub fn farenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}
fn main() {
    let celcius = 20.5;
    let fahrenheit = celcius_to_fahrenheit(celcius);
    println!("{} ℃ is {} ℉", celcius, fahrenheit);

    let fahrenheit = 75.0;
    let celcius = fahrenheit_to_celcius(fahrenheit);
    println!("{} ℉ is {} ℃", fahrenheit, celcius);
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
    32.0 + ((c*9.0)/5.0)
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}
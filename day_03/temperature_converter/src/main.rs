use std::io;

fn main() {
    println!("~~~~~~~~~~~~~~ Welcome to the temperature converter ~~~~~~~~~~~~~~");

    println!("Enter temperature in celcius");

    let mut input_temperature = String::new();
    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Failed to read line");

    let temperature: f64 = match input_temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please enter a number!"),
    };

    println!("You input: {}", temperature);

    let temperature_in_farenheit = temperature * 9.0 / 5.0 + 32.0;

    println!("Temperature in Farenheit: {}", temperature_in_farenheit);
}

mod converter;

use std::io;

fn main() {
    println!("=== NMA Weather Conversion System ===");
    println!("Select conversion type:");
    println!("1. Celsius → Fahrenheit");
    println!("2. Fahrenheit → Celsius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    println!("Enter the temperature value:");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read input");
    let value: f64 = value.trim().parse().expect("Please enter a valid number");

    match choice {
        "1" => {
            let result = converter::celsius_to_fahrenheit(value);
            println!("{value}°C = {result:.2}°F");
        }
        "2" => {
            let result = converter::fahrenheit_to_celsius(value);
            println!("{value}°F = {result:.2}°C");
        }
        _ => println!("Invalid choice! Please select 1 or 2."),
    }
}

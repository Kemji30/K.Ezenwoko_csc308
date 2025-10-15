use std::io;

fn calculate_bill(units: f64) -> f64 {
    if units > 200.0 {
        units * 30.0
    } else if units > 100.0 {
        units * 25.0
    } else {
        units * 20.0
    }
}

fn main() {
    println!("=== Smart Energy Company (SEC) Billing Calculator ===");
    println!("Please enter your electricity usage in kWh:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let units: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a number.");
            return;
        }
    };

    let total_bill = calculate_bill(units);

    println!("-------------------------------");
    println!("Electricity Usage: {:.2} kWh", units);
    println!("Total Bill: ₦{:.2}", total_bill);
    println!("-------------------------------");
    println!("Thank you for using Smart Energy Company!");
}

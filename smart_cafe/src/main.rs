use std::io;

fn calculate_discount(amount: f64) -> f64 {
    if amount > 10_000.0 {
        amount * 0.15
    } else if amount > 5_000.0 {
        amount * 0.10
    } else {
        0.0
    }
}

fn main() {
    println!("=== Smart Cafe Billing System ===");

    // Ask for total bill amount
    println!("Enter the total bill amount (₦):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let amount: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a number.");
            return;
        }
    };

    // Calculate discount
    let discount = calculate_discount(amount);
    let final_amount = amount - discount;

    println!("-------------------------------");
    println!("Original Amount: ₦{:.2}", amount);
    println!("Discount Applied: ₦{:.2}", discount);
    println!("Final Amount: ₦{:.2}", final_amount);
    println!("-------------------------------");
    println!("Thank you for visiting Smart Cafe!");
}

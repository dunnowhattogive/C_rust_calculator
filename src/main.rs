mod mathlib;
use std::env;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are provided
    if args.len() != 4 {
        eprintln!("Usage: {} <operation> <num1> <num2>", args[0]);
        eprintln!("Operations: add, subtract, multiply, divide");
        return;
    }

    let operation = &args[1];
    let num1: i32 = args[2].parse().expect("Please provide a valid integer for num1");
    let num2: i32 = args[3].parse().expect("Please provide a valid integer for num2");

    let result: f64 = match operation.as_str() {
        "add" => mathlib::add_safe(num1, num2) as f64,
        "subtract" => mathlib::subtract_safe(num1, num2) as f64,
        "multiply" => mathlib::multiply_safe(num1, num2) as f64,
        "divide" => match mathlib::divide_safe(num1, num2) {
            Some(q) => q,
            None => {
                eprintln!("Error: Division by zero");
                return;
            }
        },
        _ => {
            eprintln!("Unknown operation: {}", operation);
            return;
        }
    };

    println!("Result: {}", result);
}

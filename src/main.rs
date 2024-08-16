mod mathlib;

fn main() {
    let a = 10;
    let b = 2;

    let sum = mathlib::add(a, b);
    let difference = mathlib::subtract(a, b);
    let product = mathlib::multiply(a, b);
    let quotient = match mathlib::divide(a, b) {
        Some(q) => q,
        None => {
            eprintln!("Division by zero!");
            return;
        }
    };

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}

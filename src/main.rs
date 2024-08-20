mod mathlib;

fn main() {
    let a = 10;
    let b = 2;

    let sum = mathlib::add_safe(a, b);
    let difference = mathlib::subtract_safe(a, b);
    let product = mathlib::multiply_safe(a, b);
    let quotient = match mathlib::divide_safe(a, b) {
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

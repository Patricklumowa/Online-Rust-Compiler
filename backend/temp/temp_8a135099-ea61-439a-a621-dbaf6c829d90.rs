use std::io; // Import the input/output library

fn main() {
    println!("--- Rust Addition Calculator ---");

    // 1. Create a mutable variable to store the first input
    println!("Enter the first number:");
    let mut input1 = String::new();
    
    // 2. Read user input from the standard input (keyboard)
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    // 3. Create a mutable variable for the second input
    println!("Enter the second number:");
    let mut input2 = String::new();
    
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // 4. Convert the Strings to Integers (i32)
    // We must .trim() to remove the "Enter" key newline character
    let number1: i32 = input1.trim().parse().expect("Please type a number!");
    let number2: i32 = input2.trim().parse().expect("Please type a number!");

    // 5. Perform the math
    let result = number1 + number2;

    // 6. Output the result
    println!("The sum of {} + {} is: {}", number1, number2, result);
}
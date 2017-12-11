use std::io::{stdin};

fn main() {
    // Ask the user for input
    println!("Please enter the input number:");

    let mut input = String::new();
    stdin().read_line(&mut s).expect("Please enter a valid input number");

    let mut iterator = input.trim().chars().peekable();

    // Get the first number from the input
    let first = iterator.peek().expect("Invalid input number entered").clone();

    // The sum of the captcha
    let mut sum = 0;

    loop {
        // Get the current value
        let val = iterator.next();
        if val.is_none() {
            break;
        }
        let val = val.unwrap().clone();

        // Get the next value
        let other = match iterator.peek() {
            Some(x) => x,
            None => &first,
        };

        // If the two values are equal, add the number to the sum
        if val == *other {
            sum += val.to_digit(10).expect("Invalid input number entered");
        }
    }

    // Print the final result
    println!("The captcha sum is: {}", sum);
}

use std::io::{stdin};

fn main() {
    // Get the user input
    println!("Please enter the input number:");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Please enter a valid input number");

    // Create an iterator
    let mut iterator = input
        .trim()
        .chars()
        .peekable();

    // Remember the first number
    let first = iterator
        .peek()
        .expect("Invalid input number entered")
        .clone();

    // Get the total result sum
    let mut sum = 0;

    // Loop through the iterator
    while let Some(current) = iterator.next() {
        // Get the next value, or the first if it was the last item
        let next = match iterator.peek() {
            Some(x) => x,
            None => &first,
        };

        // Add to the sum if the value is the same as the next
        if current == *next {
            sum += current
                .to_digit(10)
                .expect("Invalid input number entered");
        }
    }

    // Print the result
    println!("The result is: {}", sum);
}

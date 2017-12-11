use std::io::stdin;

fn main() {
    // Get the user input
    println!("Please enter the input number:");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Please enter a valid input number");
    let input = input.trim();

    // Get the number size
    let size = input.len();

    // Zip two number iterators, the second one is shifted half the size
    let mut iterator = input
        .chars()
        .zip(
            input.chars()
                .cycle()
                .skip(size / 2)
        );

    // Define a variable for the sum
    let mut sum = 0;

    // Loop through the list
    for _ in 0..size {
        // Get the current and other numbers
        let (current, other) = iterator.next().unwrap();

        // Add to the sum if the value is the same as the next
        if current == other {
            sum += current
                .to_digit(10)
                .expect("Invalid input number entered");
        }
    }

    // Print the result
    println!("The result is: {}", sum);
}

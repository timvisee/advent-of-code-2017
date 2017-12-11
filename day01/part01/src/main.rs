use std::io::stdin;

fn main() {
    // Get the user input
    println!("Please enter the input number:");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Please enter a valid input number");

    // Calculate and print the result
    println!("The result is: {}", captcha(input));
}

/// Calculate the captcha value as defined by the challenge.
fn captcha(input: String) -> u32 {
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

    // Get the total result result
    let mut result = 0;

    // Loop through the iterator
    while let Some(current) = iterator.next() {
        // Get the next value, or the first if it was the last item
        let next = match iterator.peek() {
            Some(x) => x,
            None => &first,
        };

        // Add to the result if the value is the same as the next
        if current == *next {
            result += current
                .to_digit(10)
                .expect("Invalid input number entered");
        }
    }

    result
}



#[test]
fn example_one() {
    assert_eq!(captcha(String::from("1122")), 3)
}

#[test]
fn example_two() {
    assert_eq!(captcha(String::from("1111")), 4)
}

#[test]
fn example_three() {
    assert_eq!(captcha(String::from("1234")), 0)
}

#[test]
fn example_four() {
    assert_eq!(captcha(String::from("91212129")), 9)
}

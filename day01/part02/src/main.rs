use std::io::stdin;

fn main() {
    // Get the user input
    println!("Please enter the input number:");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Please enter a valid input number");

    // Print the result
    println!("The result is: {}", captcha(input));
}

/// Calculate the captcha result as defined by the challenge.
fn captcha(input: String) -> i32 {
    // Trim the input
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

    // Define a variable for the result
    let mut result = 0;

    // Loop through the list
    for _ in 0..size {
        // Get the current and other numbers
        let (current, other) = iterator.next().unwrap();

        // Add to the result if the value is the same as the next
        if current == other {
            result += current
                .to_digit(10)
                .expect("Invalid input number entered");
        }
    }

    result
}



#[test]
fn example_one() {
    assert_eq!(captcha(String::from("1212")), 6)
}

#[test]
fn example_two() {
    assert_eq!(captcha(String::from("1221")), 0)
}

#[test]
fn example_three() {
    assert_eq!(captcha(String::from("123425")), 4)
}

#[test]
fn example_four() {
    assert_eq!(captcha(String::from("123123")), 12)
}

#[test]
fn example_five() {
    assert_eq!(captcha(String::from("12131415")), 4)
}

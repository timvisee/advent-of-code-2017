use std::cmp::{min, max};
use std::io::stdin;

fn main() {
    // Ask the user for input
    println!("Please enter the sheet of numbers, followed by an empty line:");

    // Create a list of rows
    let mut rows = Vec::new();

    // Loop through each input row
    loop {
        // Get the user input
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Please enter a valid row");
        let input = input.trim().to_string();

        // If the input is empty, break
        if input.is_empty() {
            break;
        }

        // Add the row to the list
        rows.push(input);
    }

    // Print the result
    println!("The result is: {}", differences(rows));
}

/// Calculate the sum of the difference on each given row,
/// as defined by the challenge.
fn differences(rows: Vec<String>) -> u16 {
    // Define a variable for the result
    let mut result = 0;

    // Process each row
    for row in rows {
        // Split the row into separate numbers, parse them
        let numbers = row
            .split("\t")
            .map(|s| s.parse::<u16>().expect("Invalid row entered"));

        // Get the minimum and maximum values
        let (min, max) = numbers.min_max();

        // Add the difference to the result
        result += max - min;
    }

    result
}



trait IteratorMinMax: Iterator {
    /// Find the minimum and maximum value in an iterator.
    /// This consumes the iterator.
    ///
    /// Returns `(min, max)`.
    fn min_max(mut self) -> (Self::Item, Self::Item)
        where Self: Sized,
              Self::Item: Ord + Clone {
        // Get the first value of the iterator as folding start
        let first = self.next().unwrap();

        // Fold to the minimum and maximum value
        // First clone each value and output a tuple
        // Then fold the minimum and maximum value in the tuple to a result
        self.map(|x| (
                x.clone(),
                x
            ))
            .fold(
                (
                    first.clone(),
                    first
                ),
                |x, y| (
                    min(x.0, y.0),
                    max(x.1, y.1)
                ),
            )
    }
}

impl<I> IteratorMinMax for I where I: Iterator {}



#[test]
fn example() {
    assert_eq!(differences(vec![
        "5\t1\t9\t5".into(),
        "7\t5\t3".into(),
        "2\t4\t6\t8".into(),
    ]), 18)
}

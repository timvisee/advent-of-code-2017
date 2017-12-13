use std::cmp::{min, max};

/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!("{}", differences(&input))
    )
}

/// Calculate the sum of the difference on each given row,
/// as defined by the challenge.
fn differences(input: &str) -> u16 {
    // Split the input lines, and filter empty lines
    let rows = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    // Define a variable for the result
    let mut result = 0;

    // Process each row
    for row in rows {
        // Split the row into separate numbers, parse them
        let numbers = row
            .split_whitespace()
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
    assert_eq!(differences("
        5 1 9 5
        7 5 3
        2 4 6 8
    "), 18)
}

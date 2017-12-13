/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!("{}", divisions_sum(&input))
    )
}

/// Sum up the divisions for each row,
/// as defined by the challenge.
fn divisions_sum(input: &str) -> u16 {
    // Split into rows, skip emtpy lines
    let rows = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    // Define a variable for the result
    let mut result = 0;

    // Process each input row
    'processor: for row in rows  {
        // Split the row into separate numbers, parse and collect them
        let numbers: Vec<u16> = row
            .split_whitespace()
            .map(|s| s.parse::<u16>().expect("Invalid row entered"))
            .collect();

        // Go through all pairs
        // Find two values that device through each other,
        // resulting in a whole number, add the result
        for (i, a) in numbers.iter().enumerate() {
            for b in numbers[i + 1 .. numbers.len()].iter() {
                // Check whether b fully fits in a, add the devision if it is
                if a % b == 0 {
                    result += a / b;
                    continue 'processor;
                }

                // Check whether a fully fits in b, add the devision if it is
                if b % a == 0 {
                    result += b / a;
                    continue 'processor;
                }
            }
        }

        // Never reached, a divisable number pair must be found
        panic!("Invalid input, row contains no divisible pair");
    }

    result
}



#[test]
fn example() {
    assert_eq!(divisions_sum(vec![
        "5 9 2 8".into(),
        "9 4 7 3".into(),
        "3 8 6 5".into(),
    ]), 9)
}

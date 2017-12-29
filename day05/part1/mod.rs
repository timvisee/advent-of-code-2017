/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!(
            "{}",
            trace(input),
        )
    )
}

/// Trace the input, and check how many steps it take to step out of the input.
///
/// This is as defined by the challenge.
fn trace(lines: String) -> usize {
    // Get each input line
    let lines = lines.lines()
        .map(
            |line| line.trim().to_string()
        )
        .filter(
            |line| !line.is_empty()
        )
        .collect();

    // Parse and trace
    parse_trace(lines)
}

// Parse the input values from a vector, and trace.
// The amount of jumps it took to get out of this trace is returned as result.
fn parse_trace(input: Vec<String>) -> usize {
    // Parse each value as integer, and collect it in a list
    let mut values: Vec<isize> = input
        .iter()
        .map(
            |value| value
                .parse()
                .expect("Unable to parse number from input")
        )
        .collect();

    // Define the current index, and set up a counter
    let mut index = 0;
    let mut count = 0;

    // Loop until we broke out of the range
    while index >= 0 && index < values.len() as isize {
        // Increase the value
        values[index as usize] += 1;

        // Jump the index
        index += values[index as usize] - 1;

        // Increase the counter
        count += 1;
    }

    // Return the count
    count
}



#[test]
fn example_one() {
    assert_eq!(trace("
        0
        3
        0
        1
        -3
    ".into()), 5)
}

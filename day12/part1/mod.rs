use regex::Regex;

/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!("{}", count_reachable(&input))
    )
}

/// Count the reachable channels from channel 0.
/// Channel 0 itself is also included in the count.
///
/// The total reachable number is returned.
///
/// This is as defined by the challenge.
fn count_reachable(data: &str) -> usize {
    // Parse the channels
    let channels = parse_channels(data);

    // Create a list of reached, and queued channels to cover
    let mut reached = Vec::new();
    let mut queue = vec![0];

    // Loop through the list of channels, and cover them
    while let Some(current) = queue.pop() {
        // Get the newly reachable channels
        let new_reachable: Vec<usize> = channels
            .get(current)
            .unwrap()
            .iter()
            .cloned()
            .filter(|id| !reached.contains(id))
            .collect();

        // Add the newly reached channels
        reached.extend(
            new_reachable
                .iter()
                .cloned()
        );

        // Collect the items that should be queued, and add them to the queue
        let mut new_queue = new_reachable
            .iter()
            .filter(|id| !queue.contains(id))
            .cloned()
            .collect();
        queue.append(&mut new_queue);
    }

    // Return the number of reached channels
    reached.len()
}

/// Parse the data into a list of channels.
fn parse_channels(data: &str) -> Vec<Vec<usize>> {
    // Compile a regex for reading the data
    let re = Regex::new(r"([0-9]+) *<-> *(([0-9]+ *, *)*[0-9]+)").unwrap();

    // Parse the data as channels
    data.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            // Capture the data parts for the line
            let captures = re
                .captures(line.trim())
                .expect("invalid input");

            // Parse the reachable channels
            captures[2]
                .trim()
                .split(",")
                .map(|id| id
                    .trim()
                    .parse()
                    .expect("invalid reachable channels format")
                )
                .collect()
        })
        .collect()
}



#[test]
fn example_one() {
    assert_eq!(count_reachable("\
        0 <-> 2
        1 <-> 1
        2 <-> 0, 3, 4
        3 <-> 2, 4
        4 <-> 2, 3, 6
        5 <-> 6
        6 <-> 4, 5
    "), 6);
}

#[test]
fn custom_test_one() {
    assert_eq!(count_reachable("0 <-> 0"), 1);
}

use regex::Regex;

use layer::Layer;

/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!("{}", delay_for_firewall(&input))
    )
}

/// Calcualte the severity for the given firewall record.
///
/// The total severity is returned.
fn delay_for_firewall(record: &str) -> u32 {
    // Parse the layers
    let layers = parse_layers(record);

    // Get the delay that is used for the first successful pass
    (0..).filter(|delay| layers
            .iter()
            .all(|layer| !layer.is_detected_at_delay(*delay))
        )
        .next()
        .expect("failed to pass through firewall")
}

/// Parse a record to a list of layers.
fn parse_layers(record: &str) -> Vec<Layer> {
    // Compile a regex for reading the data
    let re = Regex::new(r"([0-9]+) *: *([0-9]+)").unwrap();

    // Parse the record as layer list
    record
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            // Capture the data parts for the line
            let captures = re
                .captures(line.trim())
                .expect("invalid input");

            // Create the layer
            Layer::new(
                captures[1].parse().expect("invalid input"),
                captures[2].parse().expect("invalid input"),
            )
        })
        .collect()
}



#[test]
fn example_one() {
    assert_eq!(delay_for_firewall("
        0: 3
        1: 2
        4: 4
        6: 4
    "), 10);
}

#[test]
fn custom_test_one() {
    assert_eq!(delay_for_firewall("
        0: 2
        1: 3
        2: 4
        3: 5
    "), 1);
}

#[test]
fn custom_test_two() {
    assert_eq!(delay_for_firewall(""), 0);
}

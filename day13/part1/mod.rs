use regex::Regex;

use layer::Layer;

/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!("{}", severity_for_firewall(&input))
    )
}

/// Calcualte the severity for the given firewall record.
///
/// The total severity is returned.
fn severity_for_firewall(record: &str) -> u32 {
    // Parse the layers, and determine the total severity
    parse_layers(record)
        .iter()
        .fold(
            0,
            |severity, layer| severity + layer.severity_at_depth_pos(),
        )
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
    assert_eq!(severity_for_firewall("\
        0: 3
        1: 2
        4: 4
        6: 4
    "), 24);
}

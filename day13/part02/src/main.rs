extern crate regex;

mod layer;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use layer::Layer;

// Input file
const FILE_INPUT: &'static str = "input.txt";

fn main() {
    // Get the file
    let file = File::open(FILE_INPUT);

    // Make sure the file exists
    if file.is_err() {
        println!("File {:?} not found.", FILE_INPUT);
        return;
    }

    // Read the file contents
    let mut record = String::new();
    file.unwrap()
        .read_to_string(&mut record)
        .expect("failed to read input file");

    // Calculate the delay in picoseconds to wait on the firewall to remain
    // undetected when passing through
    println!("The delay to wait is: {}", delay_for_firewall(&record));
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

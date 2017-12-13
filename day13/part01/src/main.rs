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

    // Calculate the total severity for the given firewall record
    println!("The total severity is: {}", severity_for_firewall(&record));
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
        0: 3\n\
        1: 2\n\
        4: 4\n\
        6: 4\
    "), 24);
}

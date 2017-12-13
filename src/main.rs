extern crate day01;
extern crate day02;
extern crate day11;
extern crate day12;
extern crate day13;

mod selection;

use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::path::PathBuf;
use std::str::FromStr;

use selection::Selection;



// The maximum day that may be entered
const DAY_MAX: u8 = 25;

// The maximum part that may be entered
const PART_MAX: u8 = 2;

// File constants
const FILE_INPUT_NAME: &'static str = "input.txt";
const FILE_OUTPUT_NAME: &'static str = "output.txt";

// Threshold to show an stdio input size warning
const STDIO_SIZE_WARNING_THRESHOLD: usize = 4000;



/// Application entrypoint.
fn main() {
    // Header
    println!("Advent of Code solutions by Tim Visee.");
    println!("Challenges from: https://adventofcode.com/2017\n");

    // Select the day and part to run
    let selection = select_day_part().unwrap();

    // Get input
    let input = get_input(selection);
    if input.is_err() {
        println!("Failed to get input: {}", input.unwrap_err());
        return;
    }

    // Run the challenge
    match run_challenge(selection, input.unwrap()) {
        Ok(output) => {
            // Show the output
            println!("== OUTPUT ==============");
            println!("{}", output);
        },
        Err(err) => {
            // Show an error message
            println!("An error occurred when running the challenge.");
            println!("Error: {}", err);
        },
    }
}

/// Select a day and a part to run.
fn select_day_part() -> Result<Selection, ()> {
    // Ask the user to enter a day and part
    println!("Please define what challenge to run.");

    // Select a day
    let mut day: u8 = 0;
    while day == 0 {
        // Ask to select a day
        println!("Day (1-{}):", DAY_MAX);

        // Read user input
        match read_stdin_parse() {
            Ok(input @ 1...DAY_MAX) => day = input,
            Ok(_) => println!("Day out of range"),
            _ => println!("Invalid day"),
        }
    }

    // Select a part
    loop {
        // Ask to select a part
        println!("Part (1-{}):", PART_MAX);

        // Read user input
        match read_stdin_parse() {
            Ok(input @ 1...PART_MAX) =>
                return Ok(Selection::new(day, input)),
            Ok(_) => println!("Part out of range"),
            _ => println!("Invalid part"),
        }
    }
}

/// Get the input for the given day and part
fn get_input(selection: Selection) -> Result<String, String> {
    // Get the input file path
    let input_path = get_input_file_path(selection, true);

    // Load the input from the file, if the file exists
    if input_path.is_file() {
        return Ok(
            read_file(input_path)?
        );
    }

    // The file does not exist, ask the user for input
    println!("\nManual input required, the file {:?} does not exist.", input_path);
    println!("Please provide input, end with an empty line:");

    // Create the input variable
    let mut input = String::new();

    // Retrieve input
    loop {
        // Read stdin
        let buffer = read_stdin()
            .map_err(|_| String::from("failed to read input from stdin"))?;

        // We're done if this is an empty line
        if buffer.trim_right_matches("\n").is_empty() {
            // Throw an error if the input is emtpy
            if input.is_empty() {
                return Err(String::from("no input entered"));
            }

            // Return the input buffer
            return Ok(input);
        }

        // Add the new input to the buffer
        input += &buffer;
    }
}

/// Run the given selection.
fn run_challenge(selection: Selection, input: String) -> Result<String, String> {
    // Show a running message
    println!(
        "\nRunning challenge D{:02}P{}...\n",
        selection.day(),
        selection.part()
    );

    // Run the proper challenge
    match selection.part() {
        1 => match selection.day() {
            1 => day01::part1::run(input),
            2 => day02::part1::run(input),
            11 => day11::part1::run(input),
            12 => day12::part1::run(input),
            13 => day13::part1::run(input),
            _ => Err(String::from("challenge not yet solved")),
        },
        2 => match selection.day() {
            1 => day01::part2::run(input),
            2 => day02::part2::run(input),
            11 => day11::part2::run(input),
            12 => day12::part2::run(input),
            13 => day13::part2::run(input),
            _ => Err(String::from("challenge not yet solved")),
        },
        _ => unreachable!(),
    }
}

/// Read the file contents from the given `path`.
fn read_file(path: PathBuf) -> Result<String, String> {
    // Create the file
    let mut file = File::open(path)
        .map_err(|_| String::from("failed to open input file"))?;

    // Read the file contents
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .map_err(|_| String::from("failed to read input file"))?;

    // Output the content
    Ok(buffer)
}

/// Get the file path for the given selection.
/// 
/// If `input` is `true`, the input file path is returned.
/// If it is `false`, the output file path is returned.
/// 
/// Note: this file may not exist.
fn get_input_file_path(selection: Selection, input: bool) -> PathBuf {
    PathBuf::from(
        format!(
            "./day{:02}/part{}/{}",
            selection.day(),
            selection.part(),
            if input {
                FILE_INPUT_NAME
            } else {
                FILE_OUTPUT_NAME
            },
        )
    )
}

/// Read input from stdin.
fn read_stdin() -> Result<String, ()> {
    // Create a input buffer
    let mut input = String::new();

    // Read the input
    if stdin().read_line(&mut input).is_ok() {
        // Print a warning if the input is longer than the threshold
        if input.len() >= STDIO_SIZE_WARNING_THRESHOLD {
            println!("WARNING: Your input is quite large, some parts may have been truncated by your shell.");
        }

        // Return the input
        Ok(input)
    } else {
        Err(())
    }
}

/// Read input from stdin, and get a parsed result.
fn read_stdin_parse<F>() -> Result<F, ()>
        where F: FromStr {
    // Read from stdin and parse
    read_stdin()?
        .trim_right_matches("\n")
        .parse()
        .map_err(|_| ())
}

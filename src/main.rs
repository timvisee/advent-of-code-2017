extern crate clap;
extern crate day01;
extern crate day02;
extern crate day03;
extern crate day11;
extern crate day12;
extern crate day13;

mod app;
mod arg_handler;
mod selection;

use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::path::PathBuf;
use std::str::FromStr;

use app::*;
use arg_handler::ArgHandler;
use selection::Selection;



/// Application entrypoint.
fn main() {
    // Handle arguments
    let args = ArgHandler::handle();

    // Header
    println!("{}", APP_NAME);
    println!("Solutions by {}.", APP_AUTHOR);
    println!("Challenges from https://adventofcode.com/2017\n");

    // Run the actual application logic
    run(args);
}

/// Run the application logic.
fn run(args: ArgHandler) {
    // Select the day and part to run
    let selection = select_day_part(&args).unwrap();

    // Get input
    let input = get_input(selection, &args);
    if input.is_err() {
        println!("Failed to get input: {}", input.unwrap_err());
        return;
    }

    // Run the challenge
    match run_challenge(selection, input.unwrap()) {
        Ok(output) => {
            // Get the output file path
            let output_path = get_input_file_path(selection, false);

            // Save to file
            if args.is_save() {
                // Show a status message
                println!("Writing output to {:?}...", output_path);

                // Actually write
                if let Err(err) = write_file(output.clone(), output_path) {
                    println!("Failed to write to file: {}", err);
                }

            } else {
                println!("Use --save to write the output to a file.");
            }


            // Show the output
            println!();
            print_header("Output");
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
fn select_day_part(args: &ArgHandler) -> Result<Selection, ()> {
    // Ask the user to enter a day and part
    print_header("Selection");
    println!("Please define what challenge to run.");

    // Define whether to manually select
    let mut manual = false;

    // Select a day
    let day: u8;
    loop {
        // Ask to select a day
        println!("Day (1-{}): ", DAY_MAX);

        // Get the input
        let input = if !manual && args.day().is_some() {
            let param = args.day().unwrap();
            println!("{} (selected with --day)", param);
            Ok(param)
        } else {
            read_stdin_parse()
        };

        // Read user input
        match input {
            Ok(input @ 1...DAY_MAX) => {
                day = input;
                break;
            },
            Ok(_) => println!("Day out of range"),
            _ => println!("Invalid day"),
        }

        // Enable the manual flag
        manual = true;
    }

    // Select a part
    loop {
        // Ask to select a part
        println!("Part (1-{}): ", PART_MAX);

        // Get the input
        let input = if !manual && args.part().is_some() {
            let param = args.part().unwrap();
            println!("{} (selected with --part)", param);
            Ok(param)
        } else {
            read_stdin_parse()
        };

        // Read user input
        match input {
            Ok(input @ 1...PART_MAX) =>
                return Ok(Selection::new(day, input)),
            Ok(_) => println!("Part out of range"),
            _ => println!("Invalid part"),
        }

        // Enable the manual flag
        manual = true;
    }
}

/// Get the input for the given day and part
fn get_input(selection: Selection, args: &ArgHandler) -> Result<String, String> {
    // Get the input file path
    let input_path = get_input_file_path(selection, true);

    // Try to load the file if manual mode is not enabled
    if !args.is_manual() {
        // Load the input from the file, if the file exists
        if input_path.is_file() {
            println!("\nFound input file at {:?}.", input_path);
            println!("Use --manual to force manual input mode.");
            return Ok(
                read_file(input_path)?
            );
        }

        // The file does not exist, mention it to the user
        println!();
        print_header("Input");
        println!("Manual input required, the file {:?} does not exist.", input_path);
    } else {
        // Mention that we are forcing manual input
        println!();
        print_header("Input");
        println!("Manual input mode forced with --manual.");
    }

    // Ask the user for input
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
    println!();
    print_header("Working");
    println!(
        "Running challenge D{:02}P{}...\n",
        selection.day(),
        selection.part()
    );

    // Run the proper challenge
    match selection.part() {
        1 => match selection.day() {
            1 => day01::part1::run(input),
            2 => day02::part1::run(input),
            3 => day03::part1::run(input),
            11 => day11::part1::run(input),
            12 => day12::part1::run(input),
            13 => day13::part1::run(input),
            _ => Err(String::from("challenge not yet solved")),
        },
        2 => match selection.day() {
            1 => day01::part2::run(input),
            2 => day02::part2::run(input),
            3 => day03::part2::run(input),
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

/// Write file contents to the given path.
fn write_file(output: String, path: PathBuf) -> Result<(), String> {
    // Create a new file
    let mut file = File::create(path)
        .map_err(|_| String::from("failed to open output file"))?;

    // Write the file contents
    file.write_all(output.as_bytes())
        .map_err(|_| String::from("failed to write output file"))?;

    Ok(())
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

/// Print a header
fn print_header(header: &str) {
    println!("== {} ================", header.trim().to_uppercase());
}

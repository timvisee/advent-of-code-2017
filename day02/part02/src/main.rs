use std::io::stdin;

fn main() {
    // Define a variable for the result
    let mut result = 0;

    // Ask the user for input
    println!("Please enter the sheet of numbers, followed by an empty line:");

    // Process each input row
    'processor: loop {
        // Get a new input row
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Please enter a valid row");
        let input = input.trim();

        // If the row is empty, we're done
        if input.is_empty() {
            break;
        }

        // Split the row into separate numbers, parse and collect them
        let numbers: Vec<u16> = input
            .split("\t")
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

    // Print the result
    println!("The result is: {}", result);
}

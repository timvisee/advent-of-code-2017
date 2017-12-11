use std::cmp::max;
use std::io::stdin;

fn main() {
    // Ask the user to input a path
    println!("Please enter a path to walk.");
    println!("- n: north");
    println!("- ne: north east");
    println!("- se: south east");
    println!("- s: south");
    println!("- sw: south west");
    println!("- nw: north west");
    println!("Separate each direction by a comma (,):");

    // Get the user input path
    let mut path = String::new();
    stdin()
        .read_line(&mut path)
        .expect("Please enter a valid path");
    let path = path.trim();

    // Calculate the distance and return the result
    println!("The result is: {}", hex_distance(path));
}

/// Calculate the distance in tiles to a point on a hexagonal grid.
/// The path is defined a list of directions a player would walk.
/// Each direction is separated by a comma (,).
///
/// Directions:
/// - n: north
/// - ne: north east
/// - se: south east
/// - s: south
/// - sw: south west
/// - nw: north west
///
/// This is defined by the challenge.
fn hex_distance(path: &str) -> i32 {
    // Define the position
    let mut pos: (i32, i32, i32)  = (0, 0, 0);

    // Split the path to get directions
    let dirs = path.split(",");

    // Loop through each direction
    for dir in dirs {
        // Modify the position
        pos = match dir.trim() {
            "n" => (pos.0 + 1, pos.1, pos.2),
            "ne" => (pos.0, pos.1 + 1, pos.2),
            "se" => (pos.0, pos.1, pos.2 + 1),
            "s" => (pos.0 - 1, pos.1, pos.2),
            "sw" => (pos.0, pos.1 - 1, pos.2),
            "nw" => (pos.0, pos.1, pos.2 - 1),
            dir => panic!("Invalid direction: {:?}", dir),
        };
    }

    // Get the distance, wich will be the same as the value on the axis that is
    // the furthest away from zero
    max(pos.0.abs(), max(pos.1.abs(), pos.2.abs()))
}



#[test]
fn example_one() {
    assert_eq!(hex_distance("ne,ne,ne"), 3)
}

#[test]
fn example_two() {
    assert_eq!(hex_distance("ne,ne,sw,sw"), 0)
}

#[test]
fn example_three() {
    assert_eq!(hex_distance("ne,ne,s,s"), 2)
}

#[test]
fn example_four() {
    assert_eq!(hex_distance("se,sw,se,sw,sw"), 3)
}

#[test]
fn custom_test_one() {
    assert_eq!(hex_distance("n,ne"), 2)
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Input file
const FILE_INPUT: &'static str = "input.txt";

fn main() {
    // Make sure the file exists
    if !Path::new(FILE_INPUT).is_file() {
        println!("File {:?} not found.", FILE_INPUT);
        println!("Please provide this file with the path to walk.");
        println!("- n:  north");
        println!("- ne: north east");
        println!("- se: south east");
        println!("- s:  south");
        println!("- sw: south west");
        println!("- nw: north west");
        println!("Separate each direction by a comma.");
        return;
    }

    // Get the file
    let mut file = File::open(FILE_INPUT)
        .expect("Input file not found.");

    // Read the file contents
    let mut path = String::new();
    file.read_to_string(&mut path)
        .expect("failed to read input file");

    // Calculate the distance and return the result
    println!("The result is: {}", hex_distance(&path));
}

/// Calculate the distance in tiles to a point on a hexagonal grid.
/// The path is defined a list of directions a player would walk.
/// Each direction is separated by a comma (,).
///
/// Directions:
/// - n:  north
/// - ne: north east
/// - se: south east
/// - s:  south
/// - sw: south west
/// - nw: north west
///
/// This is defined by the challenge.
fn hex_distance(path: &str) -> i32 {
    // Define a 3D position, used as isometric cube coordinate
    let mut pos: (i32, i32) = (0, 0);

    // Walk each direction
    for dir in path.split(",") {
        move_position(&mut pos, dir);
    }

    // Find the maximum as distance
    (pos.0.abs() + pos.1.abs() + (pos.0 + pos.1).abs()) / 2
}

/// Move the given 3D `pos` to the given `dir`.
///
/// This is using isometric cube coordinates, see:
/// - https://www.redblobgames.com/grids/hexagons/
/// - http://www-cs-students.stanford.edu/~amitp/Articles/Hexagon2.html
///
/// The position is modified in-place.
fn move_position(pos: &mut (i32, i32), dir: &str) {
    // Modify the position
    *pos = match dir.trim() {
        "n"  => (pos.0    , pos.1 + 1),
        "ne" => (pos.0 + 1, pos.1    ),
        "se" => (pos.0 + 1, pos.1 - 1),
        "s"  => (pos.0    , pos.1 - 1),
        "sw" => (pos.0 - 1, pos.1    ),
        "nw" => (pos.0 - 1, pos.1 + 1),
        dir  => panic!("Invalid direction: {:?}", dir),
    };
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
    assert_eq!(hex_distance("n,ne"), 2) }

#[test]
fn custom_test_two() {
    assert_eq!(hex_distance("se,se,se,se,s,s,nw,s,sw,se"), 7)
}

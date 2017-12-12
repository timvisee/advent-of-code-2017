use std::fs::File;
use std::io::prelude::*;

// Input file
const FILE_INPUT: &'static str = "input.txt";

fn main() {
    // Get the file
    let file = File::open(FILE_INPUT);

    // Make sure the file exists
    if file.is_err() {
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

    // Read the file contents
    let mut path = String::new();
    file.unwrap()
        .read_to_string(&mut path)
        .expect("failed to read input file");

    // Get the distance for the path
    println!("The result is: {}", dist(&path));
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
fn dist(path: &str) -> i32 {
    // Walk and get the final position
    let pos = path
        .split(",")
        .fold(
            (0, 0),
            |pos, dir| move_pos(pos, dir)
        );

    // Calcualte the distance to the final position
    (pos.0.abs() + pos.1.abs() + (pos.0 + pos.1).abs()) / 2
}

/// Move the given 3D `pos` to the given `dir`.
///
/// This is using isometric cube coordinates, see:
/// - https://www.redblobgames.com/grids/hexagons/
/// - http://www-cs-students.stanford.edu/~amitp/Articles/Hexagon2.html
///
/// The new position is returned.
fn move_pos(pos: (i32, i32), dir: &str) -> (i32, i32) {
    match dir.trim() {
        "n"  => (pos.0    , pos.1 + 1),
        "ne" => (pos.0 + 1, pos.1    ),
        "se" => (pos.0 + 1, pos.1 - 1),
        "s"  => (pos.0    , pos.1 - 1),
        "sw" => (pos.0 - 1, pos.1    ),
        "nw" => (pos.0 - 1, pos.1 + 1),
        dir  => panic!("Invalid direction: {:?}", dir),
    }
}



#[test]
fn example_one() {
    assert_eq!(dist("ne,ne,ne"), 3)
}

#[test]
fn example_two() {
    assert_eq!(dist("ne,ne,sw,sw"), 0)
}

#[test]
fn example_three() {
    assert_eq!(dist("ne,ne,s,s"), 2)
}

#[test]
fn example_four() {
    assert_eq!(dist("se,sw,se,sw,sw"), 3)
}

#[test]
fn custom_test_one() {
    assert_eq!(dist("n,ne"), 2) }

#[test]
fn custom_test_two() {
    assert_eq!(dist("se,se,se,se,s,s,nw,s,sw,se"), 7)
}

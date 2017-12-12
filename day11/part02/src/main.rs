use std::cmp::max;
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
    println!("The result is: {}", walk_dist_max(&path));
}

/// Calculate the maximum distance in tiles reached on a hexagonal grid.
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
fn walk_dist_max(path: &str) -> i32 {
    // Walk and get the final position
    path.split(",")
        .fold(
            ((0, 0), 0),
            |data, dir| {
                // Walk to the new position
                let pos = move_pos(data.0, dir);

                // Find the distance
                let dist = max(dist(&pos), data.1);

                // Return
                (pos, dist)
            }
        ).1
}

/// Find the tile distance to a hexagonal point
fn dist(pos: &(i32, i32)) -> i32 {
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
    assert_eq!(walk_dist_max("ne,ne,ne"), 3)
}

#[test]
fn example_two() {
    assert_eq!(walk_dist_max("ne,ne,sw,sw"), 2)
}

#[test]
fn example_three() {
    assert_eq!(walk_dist_max("ne,ne,s,s"), 2)
}

#[test]
fn example_four() {
    assert_eq!(walk_dist_max("se,sw,se,sw,sw"), 3)
}

#[test]
fn custom_test_one() {
    assert_eq!(walk_dist_max("n,ne"), 2) }

#[test]
fn custom_test_two() {
    assert_eq!(walk_dist_max("se,se,se,se,s,s,nw,s,sw,se"), 7)
}

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
    let mut pos: (i32, i32) = (0, 0);

    // Split the path to get directions
    let dirs = path.split(",");

    // Loop through each direction
    for dir in dirs {
        move_position(&mut pos, dir);
    }

    // Calculate the distance and return
    test(pos, (0, 0))
}

fn test(a: (i32, i32), b: (i32, i32)) -> i32 {
    let du = b.1 - a.1;
    let dv = (b.0 + (b.1 as f32 / 2f32).floor() as i32) 
        - (a.0 + (a.1 as f32 / 2f32).floor() as i32);

    if sign_equals(du, dv) {
        max(du.abs(), dv.abs())
    } else {
        du.abs() + dv.abs()
    }
}

/// Move the given `pos` to the given `dir`.
///
/// The position uses 2D axis, while the direction is on a hexagonal grid.
///
/// The position is modified in-place.
fn move_position(pos: &mut (i32, i32), dir: &str) {
    // Determine whether the x axis is even
    let even = pos.1 % 2 == 0;

    // Modify the position
    *pos = match dir.trim() {
        "n" => (pos.0 - 1, pos.1),
        "ne" => if even {
                    (pos.0, pos.1 + 1)
                } else {
                    (pos.0 - 1, pos.1 + 1)
                },
        "se" => if even {
                    (pos.0 + 1, pos.1 + 1)
                } else {
                    (pos.0, pos.1 + 1)
                },
        "s" => (pos.0 + 1, pos.1),
        "sw" => if even {
                    (pos.0 + 1, pos.1 - 1)
                } else {
                    (pos.0, pos.1 - 1)
                },
        "nw" => if even {
                    (pos.0, pos.1 - 1)
                } else {
                    (pos.0 - 1, pos.1 - 1)
                },
        dir => panic!("Invalid direction: {:?}", dir),
    };
}

/// Check whether the signs of two numbers are equal.
/// 
/// True if both numbers are positive or negative.
fn sign_equals(a: i32, b: i32) -> bool {
    (a == a.abs()) == (b == b.abs())
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

#[test]
fn custom_test_two() {
    assert_eq!(hex_distance("se,se,se,se,s,s,nw,s,sw,se"), 7)
}

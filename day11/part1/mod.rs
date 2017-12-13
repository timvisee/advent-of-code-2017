/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!("{}", walk_dist_end(&input))
    )
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
fn walk_dist_end(path: &str) -> i32 {
    // Walk and get the final position
    let pos = path
        .split(",")
        .fold(
            (0, 0),
            |pos, dir| move_pos(pos, dir)
        );

    // Get the distance to the point
    dist(pos)
}

/// Find the tile distance to a hexagonal point
fn dist(pos: (i32, i32)) -> i32 {
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
    assert_eq!(walk_dist_end("ne,ne,ne"), 3)
}

#[test]
fn example_two() {
    assert_eq!(walk_dist_end("ne,ne,sw,sw"), 0)
}

#[test]
fn example_three() {
    assert_eq!(walk_dist_end("ne,ne,s,s"), 2)
}

#[test]
fn example_four() {
    assert_eq!(walk_dist_end("se,sw,se,sw,sw"), 3)
}

#[test]
fn custom_test_one() {
    assert_eq!(walk_dist_end("n,ne"), 2) }

#[test]
fn custom_test_two() {
    assert_eq!(walk_dist_end("se,se,se,se,s,s,nw,s,sw,se"), 7)
}

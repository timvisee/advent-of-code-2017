/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    // Trim the input, and try to parse as number (index)
    let index = input
        .trim()
        .parse()
        .map_err(|_| String::from("invalid input"))?;

    // Calcualte the Ulam distance, and return
    Ok(
        format!(
            "{}",
            ulam_distance(index)
        )
    )
}

/// Calcualte the distance to a point on an Ulam spiral,
/// based on the given index on the spiral.
///
/// Index `1` is the center of the spiral.
///
/// This method calcualtes the answer as defined by the challenge.
fn ulam_distance(index: u32) -> u32 {
    // Determine the position on the spiral
    let pos = ulam_position(index);

    // Calcualte the Manhattan distance, and return
    pos.0.abs() as u32 + pos.1.abs() as u32
}

/// Get the position on an Ulam spiral,
/// based on the given index on the spiral.
///
/// Index `1` is the center of the spiral.
///
/// The result is returned in the format `(x, y)`.
///
/// Based on: https://math.stackexchange.com/a/163101/79567
fn ulam_position(index: u32) -> (i32, i32) {
    // Use the index as signed number
    let index = index as i32;

    let k = (((index as f64).sqrt() - 1f64) / 2f64).ceil() as i32;
    let mut t = 2 * k + 1;
    let mut m = t * t;
    t -= 1;

    if index >= m - t {
        return (k - (m - index), -k);
    }

    m = m - t;

    if index >= m - t {
        return (-k, -k + (m - index));
    }

    m = m - t;

    if index >= m - t {
        (-k + (m - index), k)
    } else {
        (k, k - (m - index - t))
    }
}



#[test]
fn example_one() {
    assert_eq!(ulam_distance(1), 0)
}

#[test]
fn example_two() {
    assert_eq!(ulam_distance(12), 3)
}

#[test]
fn example_three() {
    assert_eq!(ulam_distance(23), 2)
}

#[test]
fn example_four() {
    assert_eq!(ulam_distance(1024), 31)
}

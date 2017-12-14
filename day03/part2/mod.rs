use std::collections::HashMap;

/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    // Trim the input, and try to parse as number
    let value = input
        .trim()
        .parse()
        .map_err(|_| String::from("invalid input"))?;

    // Get the first Ulam sum value larger than the given value
    Ok(
        format!(
            "{}",
            first_ulam_sum_larger_than(value)
        )
    )
}

/// Find the first Ulam spiral sum value that is larger than the given `value`.
/// This method creates a Ulam spiral, where each node value is based on the
/// sum of all surrounding nodes (of the nodes that are already created).
/// The spiral is created in order, and node values never change once they
/// have been created.
///
/// This is similar to a fibonacci sequence, although it is formed in a spiral
/// and uses the sum of the surrounding 8 nodes.
///
/// Index `1` is the center of the spiral.
///
/// This method calcualtes the answer as defined by the challenge.
fn first_ulam_sum_larger_than(value: u32) -> u32 {
    // Create a map to put all nodes in
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();

    // Start creating each node
    for index in 1.. {
        // Determine the position for this node
        let pos = ulam_position(index);

        // Define a variable to sum up all surrounding values in
        let mut sum = 0;

        // Set the value for the first node
        if pos.0 == 0 && pos.1 == 0 {
            sum = 1;

        } else {
            // Scan the area around the selected position
            for x in -1..2 {
                for y in -1..2 {
                    // Skip the current node position
                    if x == 0 && y == 0 {
                        continue;
                    }

                    // Add the value of the selected node if any was found
                    if let Some(value) = map.get(&(pos.0 + x, pos.1 + y)) {
                        sum += value;
                    }
                }
            }
        }

        // If the sum is larger than the given value, return it
        if sum > value {
            return sum;
        }

        // Add the node to the map
        map.insert(pos, sum);
    }

    // This should never be reached
    panic!("failed to find next ulam sum value");
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
    assert_eq!(first_ulam_sum_larger_than(1), 2)
}

#[test]
fn example_two() {
    assert_eq!(first_ulam_sum_larger_than(2), 4)
}

#[test]
fn example_three() {
    assert_eq!(first_ulam_sum_larger_than(4), 5)
}

#[test]
fn example_four() {
    assert_eq!(first_ulam_sum_larger_than(747), 806)
}

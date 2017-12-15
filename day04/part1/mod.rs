/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    // Calcualte the Ulam distance, and return
    Ok(
        format!(
            "{}",
            count_lines_without_duplicate_words(input),
        )
    )
}

/// Count the number of lines that don't have duplicate words.
fn count_lines_without_duplicate_words(lines: String) -> usize {
    // Split the input by lines, and filter empty lines
    // Filter lines with no duplicates, and count the number of lines left
    lines.lines()
        .filter(
            |line| !line.trim().is_empty()
        )
        .filter(
            |line| !has_duplicate_words(line)
        )
        .count()
}

/// Check whether the input line contains any duplicate words.  
/// `true` is returned if any duplicate words are found,
/// `false` otherwise.
///
/// Note: this method is not very efficient.
fn has_duplicate_words(input: &str) -> bool {
    // Split the input string into a word list
    let mut words: Vec<&str> = input
        .trim()
        .split(" ")
        .filter(
            |word| !word.trim().is_empty()
        )
        .collect();

    // Remember the word count
    let count = words.len();

    // Deduplicate
    words.dedup();

    // Had duplicate words if the count changed
    words.len() != count
}



#[test]
fn example_one() {
    assert_eq!(count_lines_without_duplicate_words("
        aa bb cc
        aa bb cc
        aa bb cc
        aa bb cc
        aa bb cc
    ".into()), 5)
}

#[test]
fn example_two() {
    assert_eq!(count_lines_without_duplicate_words("
        aa bb cc
        aa bb cc
        aa bb aa
        aa bb cc
        aa bb bb
    ".into()), 3)
}

#[test]
fn example_three() {
    assert_eq!(count_lines_without_duplicate_words("
        aa bb cc aa
        aa bb aa cc
        aa bb cc bb
        aa bb cc cc
        aa bb bbb b cc aaa aa
    ".into()), 0)
}

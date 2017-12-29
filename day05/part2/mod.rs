/// Entrypoint.
pub fn run(input: String) -> Result<String, String> {
    Ok(
        format!(
            "{}",
            count_lines_without_anagrams(input),
        )
    )
}

/// Count the number of lines that don't have any anagrams in them.
fn count_lines_without_anagrams(lines: String) -> usize {
    // Split the input by lines, and filter empty lines
    // Filter lines with no duplicates, and count the number of lines left
    lines.lines()
        .filter(
            |line| !line.trim().is_empty()
        )
        .filter(
            |line| !has_anagrams(line)
        )
        .count()
}

/// Check whehter the input sentence contains any anagrams.
fn has_anagrams(input: &str) -> bool {
    // Split the input string into a word list
    let words: Vec<&str> = input
        .trim()
        .split(" ")
        .filter(
            |word| !word.trim().is_empty()
        )
        .collect();

    // Remember the word count
    let count = words.len();

    // Create a buffer for checked words
    let mut buff: Vec<String> = Vec::with_capacity(count - 1);

    // Loop through all words, make sure they aren't already used
    for word in words {
        // Use owned strings
        let word = word.to_string();

        // Check for any anagrams so far
        if buff.iter().any(|e| is_anagram(e, word.as_str())) {
            return true;
        }

        // Push a word to the used list
        buff.push(word);
    }

    // No duplicates found
    return false;
}

/// Check whether two strings are anagrams of each other.
/// 
/// True is returned if string `a` and `b` are anagrams, false if not.
fn is_anagram(a: &str, b: &str) -> bool {
    // Return false if both strings have a different length
    if a.len() != b.len() {
        return false;
    }

    // Create a set of usable characters
    let mut set: Vec<char> = a.chars().collect();

    // Check whether the characters of the other word cover the complete set
    for c in b.chars() {
        // The character must be in the list
        if !set.contains(&c) {
            return false;
        }

        // Remove the used character from the set
        let index = set
            .iter()
            .position(|&e| e == c)
            .unwrap();
        set.remove(index);

        // If the set is emtpy, we have an anagram
        if set.is_empty() {
            return true;
        }
    }

    // Not an anagram
    return false;
}



#[test]
fn example_one() {
    assert_eq!(count_lines_without_anagrams("
        abcde fghij
        abcde xyz ecdab
        a ab abc abd abf abj
        iiii oiii ooii oooi oooo
        oiii ioii iioi iiio
    ".into()), 3)
}

#[test]
fn anagram_one() {
    assert!(is_anagram("abc", "acb"))
}

#[test]
fn anagram_two() {
    assert!(is_anagram("testword", "wdtroets"))
}

#[test]
fn anagram_three() {
    assert!(!is_anagram("abc", "abd"))
}

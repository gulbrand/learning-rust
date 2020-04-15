// Implement an algorithm to determine if a string has all unique characters.
// What if you cannot use additional data structures?

// What is a character? A byte? Or UTF-8? Or UTF-16?

// Let's start with the assumption that it is a byte, but since that could be wrong,
// we'll need to box that assumption. This is quite complex due to UTF encodings.
// Let's remove this constraint and only deal with ascii for now.

// what is the cardinality of the key set? Check the length of the input evidence.
// If the length of the evidence is larger than the cardinality of the key set, then
// there must be a duplicate.
// If it is shorter, have to do a deeper check.

// This problem is quite interesting if we think about the class of problems it represents.

/// If we think of a string as a stream of characters, we can start to see that this problem
/// quickly turns into a stream deduplication problem.
///
/// E.g., given a stream of 'things', remove duplicates so the duplicates aren't processed.
///
///
/// What is the cost of missing a duplicate? E.g., if the string is not deduplicated at all and
/// is nothing but the same character repeated 1000 times, what is the cost of that?
///
/// Or, if some characters are repeated, but it is statistically rare, is that ok?
///
///
use std::collections::HashSet;

fn is_unique_via_sort(candidate: &str) -> bool {
    let mut chars: Vec<char> = candidate
        .chars()
        .into_iter()
        .collect();

    chars.sort();
    let mut last_c = None;
    for c in chars {
        let current_c = Some(c);
        if last_c == current_c {
            return false;
        }
        last_c = current_c;
    }
    true
}

#[allow(dead_code)]
fn is_unique_via_set(candidate: &str) -> bool {
    let seen_chars: HashSet<char> = candidate.chars().into_iter().collect();
    return seen_chars.len() == candidate.len();
}

#[allow(dead_code, unused_variables)]
fn is_unique_via_bloom_filter(_candidate: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_empty_string_is_unique() {
        let candidate = "";
        assert_eq!(is_unique_via_sort(candidate), true);
    }

    #[test]
    fn test_that_single_char_is_unique() {
        let candidate = "a";
        assert_eq!(is_unique_via_sort(candidate), true);
    }

    #[test]
    fn test_that_duplicate_char_is_not_unique() {
        let candidate = "aa";
        assert_eq!(is_unique_via_sort(candidate), false);
    }

    #[test]
    fn test_that_empty_string_is_unique_via_set() {
        let candidate = "";
        assert_eq!(is_unique_via_set(candidate), true);
    }

    #[test]
    fn test_that_single_char_is_unique_via_set() {
        let candidate = "a";
        assert_eq!(is_unique_via_set(candidate), true);
    }

    #[test]
    fn test_that_duplicate_char_is_unique_via_set() {
        let candidate = "aa";
        assert_eq!(is_unique_via_set(candidate), false);
    }
}

fn main() {
    println!("hi");
    let candidate = "aa";
    is_unique_via_sort(candidate);
    is_unique_via_set(candidate);
}

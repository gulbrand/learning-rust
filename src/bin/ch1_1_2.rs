// Check Permutation


// Given two strings, write a method to decide if one is a permutation of the other.


// first crack: map
// if they contain the same characters and counts of each, it is a permutation of the other.
fn is_permutation(s1: &str, s2: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    fn test_that_empty_is_true() {
        let candidate1 = "";
        let candidate2 = "";
        assert_eq!(is_permutation(candidate1, candidate2), true);
    }
}

fn main() {
    println!("hello");
}


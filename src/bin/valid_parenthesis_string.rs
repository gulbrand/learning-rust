
struct Solution;

struct ParenChecker {
    paren_stack: Vec<char>,
    wild_card_stack: Vec<char>
}

impl ParenChecker {
    pub fn new() -> ParenChecker {
        ParenChecker {
            paren_stack: Vec::new(),
            wild_card_stack: Vec::new(),
        }
    }

    pub fn consume(self: &mut Self, c: char) -> bool {
        if c == '(' {
            self.paren_stack.push(c);
        } else if c == ')' {
            if let Some(last_c) = self.paren_stack.pop() {
                if last_c != '(' {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    pub fn is_empty(self: Self) -> bool {
        self.paren_stack.is_empty()
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut paren_checker = ParenChecker::new();
        for c in s.chars() {
            let result = paren_checker.consume(c);
            if result != true {
                return false;
            }
        }
        return paren_checker.is_empty();
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test() {
        let test_input = "()".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_false() {
        let test_input = "())".to_string();
        let expected = false;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn complex_test_false() {
        let test_input = "((())".to_string();
        let expected = false;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_true() {
        let test_input = "(())".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

}

pub fn main() {
    println!("valid_parenthesis_string");
}
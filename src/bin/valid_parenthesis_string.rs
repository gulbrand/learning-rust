
struct Solution;


#[allow(dead_code)]
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut lo = 0;
        let mut hi = 0;
        for c in s.chars() {
            lo += if c == '(' { 1 } else { -1 };
            hi += if c == ')' { -1 } else { 1 };
            // hi should never go < 0, that indicates under-flow of previous opening left parens.
            if hi < 0 {
                return false;
            }
            // it's ok for lo to go negative, as consuming an '*' as an opening
            // left paren is speculative and could be a mistake, but ok as long as the rest checks
            // out.
            // this covers many cases, such as one or more '*' at the end, in the middle, etc.
            lo = std::cmp::max(0, lo);
        }
        lo == 0
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
    pub fn complex_missing_open_left_parens_test_false() {
        let test_input = "((())))))".to_string();
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

    #[test]
    pub fn simple_test_with_wc_true() {
        let test_input = "((*))".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn complex_test_true() {
        let test_input = "(*))".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn complex_wc_test_true() {
        let test_input = "(**))".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn complex_wc_at_end_test_true() {
        let test_input = "(**))********************".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn complex_wc_in_middle_test_true() {
        let test_input = "(************************))".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn complex_wc_at_start_test_true() {
        let test_input = "**************(************************))".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn complex_wc_all_over_test_true() {
        let test_input = "**(**(*)*))".to_string();
        let expected = true;
        let actual =
            Solution::check_valid_string(test_input);
        assert_eq!(actual, expected);
    }
}

pub fn main() {
    println!("valid_parenthesis_string");
}
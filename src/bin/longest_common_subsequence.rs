struct Solution;

impl Solution {
    pub fn index_of(bytes: &[u8], byte: u8) -> Option<usize> {
        for i in 0..bytes.len() {
            if bytes[i] == byte {
                return Some(i);
            }
        }
        None
    }

    pub fn helper(a: &[u8], b: &[u8]) -> i32 {
        if a.len() == 0 || b.len() == 0 {
            return 0;
        }
        let in_len = if let Some(index) = Solution::index_of(b, a[0])
            { Solution::helper(&a[1..], &b[index..]) + 1 } else { 0 };
        let out_len = Solution::helper(&a[1..], b);
        return std::cmp::max(in_len, out_len);
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1_bytes = text1.as_bytes();
        let text2_bytes = text2.as_bytes();
        let mut max_common_subsequence = Solution::helper(text1_bytes, text2_bytes);
        return max_common_subsequence;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Solution;

    #[test]
    pub fn simple_test_001() {
        let input_a = String::from("");
        let input_b = String::from("");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_002() {
        let input_a = String::from("a");
        let input_b = String::from("");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_003() {
        let input_a = String::from("");
        let input_b = String::from("b");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_004() {
        let input_a = String::from("b");
        let input_b = String::from("b");
        let expected = 1;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn leetcode_test_001() {
        let input_a = String::from("abcde");
        let input_b = String::from("ace");
        let expected = 3;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn leetcode_test_002() {
        let input_a = String::from("abc");
        let input_b = String::from("abc");
        let expected = 3;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn leetcode_test_003() {
        let input_a = String::from("abc");
        let input_b = String::from("def");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

}

pub fn main() {
    println!("longest_common_subsequence");
}
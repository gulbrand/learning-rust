// #![feature(trace_macros)]
// trace_macros!(true);

struct Solution;

impl Solution {
    pub fn helper(matrix: &Vec<Vec<char>>, row: usize, col: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if row >= matrix.len() || col >= matrix[row].len() {
            return 0;
        }
        if dp[row][col] != -1 {
            return dp[row][col];
        }
        let right = Solution::helper(matrix, row, col + 1, dp);
        let down = Solution::helper(matrix, row + 1, col, dp);
        let down_right = Solution::helper(matrix, row + 1, col + 1, dp);
        let smallest_neighbor =
            std::cmp::min(right, std::cmp::min(down, down_right));
        if matrix[row][col] == '0' {
            dp[row][col] = 0;
        } else {
            let this_value = if matrix[row][col] == '0' { 0 } else { 1 } + smallest_neighbor;
            dp[row][col] = this_value;
        }
        return dp[row][col];
    }

    #[allow(unused)]
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() < 1 {
            return 0;
        }
        if matrix[0].len() < 1 {
            return 0;
        }

        let mut max_so_far = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; cols]; rows];
        max_so_far = Solution::helper(&matrix, 0, 0, &mut dp);

        for row in 0..rows {
            for col in 0..cols {
                // print!("{}", matrix[row][col]);
            }
            // println!("");
        }
        // println!("");
        for row in 0..rows {
            for col in 0..cols {
                // print!("{}", dp[row][col]);
                max_so_far = std::cmp::max(max_so_far, dp[row][col]);
            }
            // println!("");
        }
        return max_so_far * max_so_far;
    }
}

pub fn integer_to_char(x: u8) -> char {
    let c = x + ('0' as u8);
    return c as char;
}

#[allow(unused)]
macro_rules! array_to_vec {
    ( $( $array:expr ),* ) => {
        {
            let mut tmp_vec = Vec::new();
            $(
                let mut tmp_row = Vec::new();
                let input_row = $array;
                for i in 0..input_row.len() {
                    tmp_row.push(integer_to_char(input_row[i]));
                }
                tmp_vec.push(tmp_row);
            )*
            tmp_vec
        }
    };
    ( [$array:expr],* ) => {
        {
            array_to_vec!($array)
        }
    };
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test_int_to_char_001() {
        let x = 0;
        let actual = integer_to_char(x);
        assert_eq!(actual, '0');
    }

    #[test]
    pub fn simple_test_int_to_char_002() {
        let x = 1;
        let actual = integer_to_char(x);
        assert_eq!(actual, '1');
    }

    #[test]
    pub fn simple_test_001() {
        let input: Vec<Vec<char>> =
            array_to_vec![
                [0, 0],
                [0, 0],
                [0, 0],
                [0, 0]];
        let expected = 0;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_002() {
        let input =
            array_to_vec![[1]];
        let expected = 1;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_003() {
        let input =
            array_to_vec![[1,1], [1,1]];
        let expected = 2 * 2;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_004() {
        let input =
            array_to_vec![[1,1], [1,1], [1,1]];
        let expected = 2 * 2;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_005() {
        let input =
            array_to_vec![[1,1,1], [1,1,1], [1,1,1]];
        let expected = 3 * 3;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_006() {
        let input =
            array_to_vec![
            [1,1,0],
            [1,1,0],
            [1,1,0]];
        let expected = 2 * 2;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_007() {
        let input =
            array_to_vec![
            [0,0,0],
            [0,1,1],
            [0,1,1]];
        let expected = 2 * 2;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_008() {
        let input =
            array_to_vec![
            [0,0,0,0],
            [0,1,1,0],
            [0,1,1,0]];
        let expected = 2 * 2;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_009() {
        let input =
            array_to_vec![
            [1,1,0,1],
            [1,1,0,1],
            [1,1,1,1]];
        let expected = 2 * 2;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_010() {
        let input =
            array_to_vec![
            [1,0,1,1,0,1],
            [1,1,1,1,1,1],
            [0,1,1,0,1,1],
            [1,1,1,0,1,0],
            [0,1,1,1,1,1],
            [1,1,0,1,1,1]];
        let expected = 2 * 2;
        let actual = Solution::maximal_square(input);
        assert_eq!(actual, expected);
    }
}

pub fn main() {
    println!("maximal_square");
}
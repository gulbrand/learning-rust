#[allow(unused)]
struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 1 { return 0; }
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if row == 0 && col == 0 {
                    continue;
                }
                let value_above = if row == 0 { std::i32::MAX } else { grid[row-1][col] };
                let value_left = if col == 0 { std::i32::MAX } else { grid[row][col-1] };
                grid[row][col] += std::cmp::min(value_above, value_left);
            }
        }
        let rows = grid.len();
        let cols = grid[0].len();
        grid[rows-1][cols-1]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test() {
        let input = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let expected = 7;
        let actual = Solution::min_path_sum(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn bigger_test() {
        let _input = [[1, 4, 8, 6, 2, 2, 1, 7], [4, 7, 3, 1, 4, 5, 5, 1], [8, 8, 2, 1, 1, 8, 0, 1], [8, 9, 2, 9, 8, 0, 8, 9], [5, 7, 5, 7, 1, 8, 5, 5], [7, 0, 9, 4, 5, 6, 5, 6], [4, 9, 9, 7, 9, 1, 9, 0]];
        let mut input: Vec<Vec<i32>> = Vec::new();
        for i in 0.._input.len() {
            let mut _row: Vec<i32> = Vec::new();
            for j in 0.._input[i].len() {
                _row.push(_input[i][j]);
            }
            input.push(_row);
        }
        let expected = 47;
        let actual = Solution::min_path_sum(input);
        assert_eq!(actual, expected);
    }
}

pub fn main() {
    println!("min_path_sum");
}


struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn dfs_min_sum_of_path(
        grid: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        cache: &mut HashMap<usize, i32>) -> Option<i32> {
        if row == 0 && col == 0 {
            return Some(grid[row][col]);
        }
        let cache_key= (row*grid[0].len()) + col;
        let answer = if let Some(already_computed_answer) =
            cache.get(&cache_key) {
            Some(*already_computed_answer)
        } else {
            let up =
                if row >= 1 { Solution::dfs_min_sum_of_path(grid, row-1, col, cache) }
                else { None };
            let left =
                if col >= 1 { Solution::dfs_min_sum_of_path(grid, row, col-1, cache) }
                else { None };

            let min =
                if up.is_some() && left.is_some() {
                    std::cmp::min(up.unwrap(), left.unwrap())
                } else if up.is_some() {
                    up.unwrap()
                } else if left.is_some() {
                    left.unwrap()
                } else {
                    0
                };

            let cache_value = grid[row][col] + min;
            cache.insert(cache_key, cache_value);
            Some(cache_value)
        };
        return answer;
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 1 { return 0; }
        let rows = grid.len();
        let cols = grid[0].len();
        let mut cache: HashMap<usize, i32> = HashMap::new();
        return if let Some(answer) =
            Solution::dfs_min_sum_of_path(&grid, rows-1, cols-1, &mut cache) {
            answer
        } else {
            0
        }
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

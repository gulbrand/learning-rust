
struct Solution;

impl Solution {
    pub fn dfs_min_sum_of_path(
        grid: &Vec<Vec<i32>>,
        row: i32,
        col: i32,
        cache: &mut Vec<Vec<i32>>) -> i32 {
        if row < 0 || col < 0 {
            return std::i32::MAX;
        }
        if row == 0 && col == 0 {
            return grid[row as usize][col as usize];
        }
        let answer = if cache[row as usize][col as usize] != -1 {
            cache[row as usize][col as usize]
        } else {
            let up = Solution::dfs_min_sum_of_path(grid, row-1, col, cache);
            let left = Solution::dfs_min_sum_of_path(grid, row, col-1, cache);
            let min = std::cmp::min(up, left);

            let cache_value = grid[row as usize][col as usize] + min;
            cache[row as usize][col as usize] = cache_value;
            cache_value
        };
        return answer;
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 1 { return 0; }
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut cache: Vec<Vec<i32>> = vec![vec![-1 as i32; grid[0].len()]; grid.len()];
        let answer =
            Solution::dfs_min_sum_of_path(&grid, rows-1, cols-1, &mut cache);
        return answer;
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

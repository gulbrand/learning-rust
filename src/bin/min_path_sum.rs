struct Solution;

impl Solution {
    pub fn dfs_min_sum_of_path(grid: &Vec<Vec<i32>>, x: usize, y: usize, sum_so_far: i32) -> i32 {
        println!("checking {},{}", x, y);
        let up_value = if x >= 1 { Some(grid[x - 1][y]) } else { None };
        let left_value = if y >= 1 { Some(grid[x][y - 1]) } else { None };
        let sum =
            if up_value.is_some() && left_value.is_some() {
                if up_value.unwrap() < left_value.unwrap() {
                    Solution::dfs_min_sum_of_path(grid, x - 1, y, grid[x][y])
                } else {
                    Solution::dfs_min_sum_of_path(grid, x, y - 1, grid[x][y])
                }
            } else if up_value.is_some() {
                Solution::dfs_min_sum_of_path(grid, x - 1, y, grid[x][y])
            } else if left_value.is_some() {
                Solution::dfs_min_sum_of_path(grid, x, y - 1, grid[x][y])
            } else {
                grid[x][y]
            };

        return sum_so_far + sum;
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 1 { return 0; }
        let x = grid.len() - 1;
        let y = grid[0].len() - 1;
        return Solution::dfs_min_sum_of_path(&grid, x, y, 0);
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

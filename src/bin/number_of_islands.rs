struct Solution;

#[allow(unused)]
impl Solution {
    #[allow(unused)]
    pub fn dfs_marker(mut grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let deltas: Vec<(i32, i32)> = vec![
            (-1, 0),
            (0, -1), (0, 1),
            (1, 0)
        ];
        // find cells to visit by staying with in the bounds of the vec.
        let to_visit: Vec<(usize, usize)> = deltas.iter()
            .map(|a| (a.0 + i as i32, a.1 + j as i32))
            .filter(
                |a| if a.0 < 0 || a.0 >= grid.len() as i32
                    || a.1 < 0 || a.1 >= grid[0].len() as i32
                    || grid[a.0 as usize][a.1 as usize] != '1'
                { false } else { true })
            .map(|a| (a.0 as usize, a.1 as usize))
            .collect();

        // mark and visit.
        for x in to_visit {
            grid[x.0][x.1] = '2';
            Solution::dfs_marker(&mut grid, x.0, x.1);
        }
    }

    #[allow(unused)]
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.len() < 1 {
            return 0;
        }
        let mut island_count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let c = grid[i][j];
                if c == '1' {
                    island_count += 1;
                    Solution::dfs_marker(&mut grid, i, j);
                }
            }
        }
        return island_count;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn breaking_case() {
        let test_case =
            (
                vec![vec!['1', '1', '0', '0', '0'], vec!['1', '1', '0', '0', '0'], vec!['0', '0', '1', '0', '0'], vec!['0', '0', '0', '1', '1']],
                3
            );

        let actual = Solution::num_islands(test_case.0);
        assert_eq!(actual, test_case.1);
    }
}

pub fn main() {
    println!("number_of_islands");
}
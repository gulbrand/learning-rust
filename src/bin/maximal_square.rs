struct Solution;

impl Solution {

    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() < 1 {
            return 0;
        }
        if matrix[0].len() < 1 {
            return 0;
        }
        let n = std::cmp::min(matrix.len(), matrix[0].len());
        println!("working with n = {}", n);
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                print!("{}", matrix[i][j]);
            }
            println!("");
        }
        println!("");

        let mut max_so_far = 0;
        let mut row_run_counts: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut col_run_counts: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            let mut running_square_count = 0;
            for j in 0..matrix[i].len() {
                if matrix[i][j] == '1' {
                    running_square_count += 1;
                } else {
                    running_square_count = 0;
                }
                row_run_counts[i][j] = running_square_count;
            }
        }

        for j in 0..matrix[0].len() {
            let mut running_square_count = 0;
            for i in 0..matrix.len() {
                if matrix[i][j] == '1' {
                    running_square_count += 1;
                } else {
                    running_square_count = 0;
                }
                col_run_counts[i][j] = running_square_count;
            }
        }
        println!("{:?}", row_run_counts);
        for i in 0..row_run_counts.len() {
            for j in 0..row_run_counts[i].len() {
                print!("{}", row_run_counts[i][j]);
            }
            println!("");
        }
        println!("");
        println!("{:?}", col_run_counts);
        for i in 0..col_run_counts.len() {
            for j in 0..col_run_counts[i].len() {
                print!("{}", col_run_counts[i][j]);
            }
            println!("");
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let row_count = row_run_counts[i][j];
                let col_count = col_run_counts[i][j];
                if row_count > 0
                    && col_count > 0 {
                    let square_edge_len_max =
                        std::cmp::min(row_run_counts[i][j], col_run_counts[i][j]);

                    max_so_far = std::cmp::max(max_so_far, square_edge_len_max);
                }
            }
        }

        return max_so_far;
    }
}




pub fn main() {
    println!("maximal_square");
}
struct NaiveSolution {}

impl NaiveSolution {

    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut max_distance = 0;

        for i in 0..arrays.len() {
            for j in i+1..arrays.len() {
                max_distance = std::cmp::max(
                    max_distance,
                    std::cmp::max(
                        (arrays[i][0] - arrays[j][arrays[j].len() - 1]).abs(),
                        (arrays[j][0] - arrays[i][arrays[i].len() - 1]).abs(),
                    )
                );
            }
        }
        return max_distance;
    }
}

struct FastestSolution {}
impl FastestSolution {

    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut min = arrays[0][0];
        let mut max = arrays[0][arrays[0].len() - 1];
        for i in 1..arrays.len() {
            res = std::cmp::max(
                res,
                std::cmp::max(
                    (min - arrays[i][arrays[i].len()-1]).abs(),
                    (max - arrays[i][0]).abs()
                )
            );
            min = std::cmp::min(min, arrays[i][0]);
            max = std::cmp::max(max, arrays[i][arrays[i].len() - 1]);
        }
        return res;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn fastest_easy() {
        let expected = 4;
        let input = vec![vec![1,2,3], vec![4,5], vec![1,2,3]];

        let actual = FastestSolution::max_distance(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn fastest_with_negative() {
        let expected = 6;
        let input = vec![vec![-1,2], vec![4,5], vec![1,2,3]];

        let actual = FastestSolution::max_distance(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn naive() {
        let expected = 4;
        let input = vec![vec![1,2,3], vec![4,5], vec![1,2,3]];

        let actual = NaiveSolution::max_distance(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn naive_with_negative() {
        let expected = 6;
        let input = vec![vec![-1,2], vec![4,5], vec![1,2,3]];

        let actual = NaiveSolution::max_distance(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn naive_longer() {
        let expected = 8;
        let input = vec![
            vec![1,2,3],
            vec![4,5],
            vec![1,2,3],
            vec![7,8,9],
            vec![4,5],
            vec![1,2]
        ];

        let actual = NaiveSolution::max_distance(input);
        assert_eq!(actual, expected);
    }
}

fn main() {
    println!("max distance in arrays");
}

struct DPSolution {}

impl DPSolution {
    fn rob_simple(nums: &[i32]) -> i32 {
        let mut t1 = 0;
        let mut t2 = 0;
        for current_num in nums {
            let tmp = t1;
            t1 = std::cmp::max(current_num + t2, t1);
            t2 = tmp;
        }
        t1
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        if nums.len() < 2 {
            return nums[0];
        }
        let n = nums.as_slice();
        let max_sum = std::cmp::max(
            DPSolution::rob_simple(&n[1..]),
            DPSolution::rob_simple(&n[0..n.len()-1])
        );
        max_sum
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn empty() {
        let input = vec![];
        let expected: i32 = 0;
        let actual = DPSolution::rob(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn zero() {
        let input = vec![0];
        let expected: i32 = 0;
        let actual = DPSolution::rob(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn one() {
        let input = vec![1];
        let expected: i32 = 1;
        let actual = DPSolution::rob(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn two() {
        let input = vec![1, 2];
        let expected: i32 = 2;
        let actual = DPSolution::rob(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn three() {
        let input = vec![1, 2, 3];
        let expected: i32 = 3;
        let actual = DPSolution::rob(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn large() {
        let input = vec![2, 3, 4, 5, 6];
        let expected: i32 = 10;
        let actual = DPSolution::rob(input);
        assert_eq!(actual, expected);
    }

}

pub fn main() {
    println!("house robber ii");
}
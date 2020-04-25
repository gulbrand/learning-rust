struct Solution;

impl Solution {

    pub fn can_reach_the_end(nums: &Vec<i32>, index: usize) -> bool {
        if (index == nums.len() - 1) {
            return true;
        }
        let max_jump_index = std::cmp::min(index + nums[index] as usize, nums.len() - 1);
        for i in (index+1..=max_jump_index).rev() {
            if (Solution::can_reach_the_end(nums, i)) {
                return true;
            }
        }
        // println!("{:?}, {}", nums, index);
        return false;
    }

    pub fn can_jump(nums: Vec<i32>) -> bool {
        return Solution::can_reach_the_end(&nums, 0);
    }
}

#[cfg(test)]
pub mod test {
    use crate::Solution;

    #[test]
    pub fn simple_test_0() {
        let input = vec![0];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_01() {
        let input = vec![1,0];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_02() {
        let input = vec![2,0,6,9,8,4,5,0,8,9,1,2,9,6,8,8,0,6,3,1,2,2,1,2,6,5,3,1,2,2,6,4,2,4,3,0,0,0,3,8,2,4,0,1,2,0,1,4,6,5,8,0,7,9,3,4,6,6,5,8,9,3,4,3,7,0,4,9,0,9,8,4,3,0,7,7,1,9,1,9,4,9,0,1,9,5,7,7,1,5,8,2,8,2,6,8,2,2,7,5,1,7,9,6];
        let expected: bool = false;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test() {
        let input = vec![2,1];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_1() {
        let input = vec![1,1];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_2() {
        let input = vec![1,1,1];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_3() {
        let input = vec![1,0,1];
        let expected: bool = false;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_4() {
        let input = vec![2,0,1];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_5() {
        let input = vec![3,0,0,1];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_6() {
        let input = vec![3,2,1,0,1];
        let expected: bool = false;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_7() {
        let input = vec![100,0,0,0,0,0,0,0,0,1];
        let expected: bool = true;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_8() {
        let input = vec![0,2,3];
        let expected: bool = false;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_9() {
        let input = vec![1,0,1,0];
        let expected: bool = false;
        let actual = Solution::can_jump(input);
        assert_eq!(actual, expected);
    }

}

pub fn main() {
    println!("jump_game");
}
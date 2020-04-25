struct Solution;

impl Solution {
    pub fn helper(nums: &[i32], required: i32) -> bool {
        println!("{:?}, {}", nums, required);
        if nums.len() < 3 {
            return
                (nums[1] >= required && nums[0] >= 1)
            ||
                    (nums[0] >= 1 && nums[0] >= required);
        }
        let second_to_last = nums[nums.len()-2];
        if second_to_last < required {
            if nums.len() < 1 {
                return false;
            }
            return Solution::helper(&nums[..nums.len()-1], required+1);
        }
        return Solution::helper(&nums[..nums.len()-1], required-1);
    }
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        if nums.len() < 3 {
            return nums[0] >= 1;
        }
        return Solution::helper(&nums[..], 1);
    }
}



#[cfg(test)]
pub mod test {
    use crate::Solution;

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
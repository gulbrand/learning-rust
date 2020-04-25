struct Solution;

impl Solution {

    pub fn bt_helper(nums: &Vec<i32>, index: usize, solution_found: &mut bool) {
        // is this the end? did we find a solution or not?
        // println!("{:?}, {}", nums, index);
        if index >= nums.len()-1 {
            *solution_found = true;
        } else {
            let mut next: Vec<usize> = Vec::new();
            let jumps = nums[index];
            for i in (1..jumps+1).rev() {
                next.push(index+i as usize);
            }
            // println!("next = {:?}", next);
            for n in next {
                Solution::bt_helper(nums, n, solution_found);
                if *solution_found {
                    return;
                }
            }
        }
    }

    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut solution_found: bool = false;
        Solution::bt_helper(&nums, 0, &mut solution_found);
        return solution_found;
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
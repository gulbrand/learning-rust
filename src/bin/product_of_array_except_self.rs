struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // Okay, let's do a 2-pass approach.
        let mut products_moving_right: Vec<i32> = Vec::new();
        let mut products_moving_left: Vec<i32> = Vec::new();
        let mut running_product = 1;
        for i in 0..nums.len() {
            products_moving_right.push(running_product);
            running_product *= nums[i];
        }

        running_product = 1;
        for i in (0..nums.len()).rev() {
            products_moving_left.push(running_product);
            running_product *= nums[i];
        }

        let right_range = (0..nums.len()).rev();
        let left_range = 0..nums.len();
        let mut answer: Vec<i32> = Vec::new();
        for (right, left) in left_range.zip(right_range) {
            answer.push(
                products_moving_right[right] * products_moving_left[left]
            );
        }

        return answer;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test() {
        let input = vec![1, 0];
        let actual = Solution::product_except_self(input);
        assert_eq!(actual, vec![0, 1]);
    }

    #[test]
    pub fn simple_test_all_zero() {
        let input = vec![0, 0];
        let actual = Solution::product_except_self(input);
        assert_eq!(actual, vec![0, 0]);
    }

    #[test]
    pub fn simple_test_bit_longer() {
        let input = vec![1, 2, 3, 4];
        let actual = Solution::product_except_self(input);
        assert_eq!(actual, vec![24, 12, 8, 6]);
    }
}


pub fn main() {
    println!("bleh");
}
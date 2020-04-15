struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut products: Vec<i32> = Vec::new();
        for (i, _) in nums.iter().enumerate() {
            let mut product = 1;
            for j in 0..nums.len() {
                if j != i {
                    product *= nums[j];
                }
            }
            products.push(product);
        }
        return products;
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

#[allow(dead_code)]
fn dfs(nums: &[i32]) -> i32 {
    println!("dfs(nums = {:?})", nums);
    let n = nums.len();
    match n {
        0 => 0,
        1 => nums[0],
        _ => {
            let l = dfs(&nums[0..(n/2)]);
            let r = dfs(&nums[(n/2)..n]);
            let max =
                std::cmp::max(
                    std::cmp::max(l, r),
                    l + r
                );
            println!("max = {} for nums = {:?}", max, nums);
            max
        }
    }
}

#[allow(dead_code)]
fn compute_max_subarray_n2(nums: &[i32]) -> i32 {
    let mut max_seen_so_far: i32 = std::i32::MIN;
    for i in 0..nums.len() as usize {
        for j in i..nums.len() as usize {
            let subarray = &nums[i..j+1];
            let sum_for_this_subarray =
                subarray.iter().fold(0, |acc, x| acc + x);
            max_seen_so_far =
                std::cmp::max(
                    max_seen_so_far,
                    sum_for_this_subarray);
        }
    }
    return max_seen_so_far;
}



fn kadane(nums: &[i32]) -> i32 {
    let mut max_so_far = 0;
    let mut max_ending_here = 0;

    for i in 0..nums.len() {
        max_ending_here += nums[i];
        if max_ending_here < 0 {
            max_ending_here = 0;
        }
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
    }
    max_so_far
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        println!("nums = {:?}", nums);
        return match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => kadane(nums.as_slice())
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let expected = 1;
        let nums = vec![-2, 1];
        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }

    #[test]
    fn med_odd_case() {
        let expected = 4;
        let nums = vec![-1, -3, 4];
        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }

    #[test]
    fn med_even_cross_node_case() {
        let expected = 7;
        let nums = vec![-10, 4, -1, 4];
        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }

    #[test]
    fn med_case() {
        let expected = 4;
        let nums = vec![-2, 1, -3, 4];
        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }

    #[test]
    fn bigger_case() {
        let expected = 6;
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];

        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }
}

pub fn main() {
    println!("hello");
}
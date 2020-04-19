struct Solution;

#[allow(unused)]
impl Solution {
    #[allow(unused)]
    pub fn search(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 1 {
            return -1;
        }
        if nums.len() < 2 {
            if nums[0] == target {
                return 0;
            }
        }
        let mut i: i32 = 0;
        let mut j: i32  = (nums.len() - 1) as i32;
        loop {
            let mid: i32 = (i + j) / 2;
            // println!("(i, mid, j) = ({}, {}, {})", i, mid as i32, j as i32);
            if j < i || j < 0 || i < 0 {
                return -1;
            }
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[i as usize] <= nums[mid as usize] {
                if target < nums[mid as usize] && target >= nums[i as usize] {
                    j = mid - 1;
                } else {
                    i = mid + 1;
                }
            } else if nums[mid as usize] < nums[i as usize] {
                if target > nums[mid as usize] && target <= nums[j as usize] {
                    i = mid + 1;
                } else {
                    j = mid - 1;
                }
            } else {
                return -1;
            }
        }
        return -1;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_tests() {
        let tests = vec![
            (vec![3,1], 3, 0),
            (vec![3,1], 1, 1),
            (vec![3,1], 0, -1),
            (vec![1,2], 0, -1),
            (vec![1,2], 1, 0),
            (vec![1,2], 2, 1),
            (vec![4,5,6,7,0,1,2,3], 2, 6),
            (vec![4,5,6,7,0,1,2,3], 0, 4),
            (vec![4,5,6,7,0,1,2,3], 7, 3),
        ];
        for test in tests {
            let input = test.0;
            let target = test.1;
            let actual = Solution::search(input, target);
            assert_eq!(actual, test.2);
        }
    }
}

pub fn main() {
    println!("search_rotated_sorted_array")
}
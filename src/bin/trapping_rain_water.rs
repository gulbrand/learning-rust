use std::borrow::{BorrowMut};

struct Solution;

impl Solution {
    pub fn compute_volume(location: (usize, i32), walls: &[(usize, i32)]) -> i32 {
        if walls.len() < 1 {
            return 0;
        }
        let mut volume = 0;
        let mut water_level: i32 = 0;
        let range = (0..walls.len()).rev();
        println!("range = {:?}", range);
        for i in range {
            if walls[i].1 <= location.1 {
                volume += (walls[i].1 - water_level)
                    * ((location.0  - walls[i].0) as i32);
                water_level += walls[i].1;
            } else if walls[i].1 > location.1 {
                volume += (location.1 - water_level) * ((location.0 - walls[i].0) as i32);
            }
        }
        return volume;
    }

    pub fn pop_walls_lower_than_this_height(height: i32, walls: &mut Vec<(usize, i32)>) {
        while !walls.is_empty() && walls[walls.len()-1].1 <= height {
            walls.pop();
        }
    }

    #[allow(dead_code)]
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut last_height = 0;
        let mut prev_walls: Vec<(usize, i32)> = Vec::new();
        for i in 0..height.len() {
            if height[i] < last_height {
                prev_walls.push((i, last_height));
            }
            if height[i] > last_height {
                result += Solution::compute_volume((i, height[i]), prev_walls.as_slice());
                Solution::pop_walls_lower_than_this_height(height[i], prev_walls.borrow_mut());
            }
            last_height = height[i];
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::borrow::BorrowMut;

    #[test]
    pub fn test_compute_volume_small() {
        test_compute_volume(
            (2, 1), vec![(0, 2)], 1);
    }

    #[test]
    pub fn test_compute_volume_small_1() {
        test_compute_volume(
            (3, 1), vec![(0, 2)], 1);
    }

    #[test]
    pub fn test_compute_volume_small_2() {
        test_compute_volume(
            (4, 1), vec![(0, 2), (2, 1)], 2);
    }

    fn test_compute_volume(
        position: (usize, i32),
        walls: Vec<(usize, i32)>,
        expected: i32) {
        println!("position = {:?}, walls = {:?}, expected = {}", position, walls, expected);
        let actual = Solution::compute_volume(position, walls.as_slice());
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn pop_test() {
        let mut walls: Vec<(usize, i32)> = Vec::new();
        walls.push((0, 1));
        Solution::pop_walls_lower_than_this_height(0, walls.borrow_mut());
        assert_eq!(walls.len(), 0);
    }

    #[test]
    pub fn pop_test_1() {
        let mut walls: Vec<(usize, i32)> = Vec::new();
        walls.push((0, 1));
        walls.push((1, 2));
        Solution::pop_walls_lower_than_this_height(3, walls.borrow_mut());
        assert_eq!(walls.len(), 0);
    }

    #[test]
    pub fn pop_test_2() {
        let mut walls: Vec<(usize, i32)> = Vec::new();
        walls.push((0, 3));
        walls.push((1, 2));
        walls.push((2, 1));
        Solution::pop_walls_lower_than_this_height(4, walls.borrow_mut());
        assert_eq!(walls.len(), 0);
    }

    #[test]
    pub fn pop_test_3() {
        let mut walls: Vec<(usize, i32)> = Vec::new();
        walls.push((0, 3));
        walls.push((1, 2));
        walls.push((2, 1));
        Solution::pop_walls_lower_than_this_height(2, walls.borrow_mut());
        assert_eq!(walls.len(), 1);
    }

    #[test]
    pub fn no_rain_test() {
        run_test(vec![0, 1, 0, 0], 0);
    }

    #[test]
    pub fn simple_test() {
        run_test(vec![0, 1, 0, 1], 1);
    }

    #[test]
    pub fn longer_test() {
        run_test(vec![0, 1, 0, 2], 1);
    }

    #[test]
    pub fn short_taller_test() {
        run_test(vec![3, 0, 2, 0, 1], 2);
    }

    #[test]
    pub fn longer_longer_test() {
        run_test(vec![0, 2, 0, 1, 0, 3], 5);
    }

    #[test]
    pub fn longer_longer_test_1() {
        run_test(vec![2, 0, 1, 0, 1], 2);
    }

    pub fn run_test(input: Vec<i32>, expected: i32) {
        let actual = Solution::trap(input);
        assert_eq!(actual, expected);
    }
}


pub fn main() {
    println!("do nothing");
}
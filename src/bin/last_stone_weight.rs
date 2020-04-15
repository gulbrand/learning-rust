use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn smash(left: i32, right: i32) -> i32 {
        if left == right {
            return 0;
        }
        return std::cmp::max(left, right) - std::cmp::min(left, right);
    }

    #[allow(unused)]
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        if stones.len() < 1 {
            return 0;
        }
        if stones.len() < 2 {
            return stones[0];
        }
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
        let mut last_stone = 0;
        loop {
            if heap.len() < 2 {
                break;
            }
            let first = heap.pop().expect("first shouldn't be None");
            let second = heap.pop().expect("second shouldn't be None");
            last_stone = Solution::smash(first, second);
            heap.push(last_stone);
        }

        return last_stone;
    }
}


#[cfg(test)]
pub mod test {

    use crate::Solution;

    #[test]
    pub fn simple_test() {
        let mut stones = vec![2,7,4,1,8,1];

        stones.sort_by(|a, b| b.cmp(a));
        println!("{:?}", stones);

        stones.sort_by(|a, b| a.cmp(b));
        println!("{:?}", stones);

        let actual = Solution::last_stone_weight(stones);
        assert_eq!(actual, 1);
    }

    #[test]
    pub fn single_element_test() {
        let mut stones = vec![1];
        stones.sort();
        println!("{:?}", stones);
        let actual = Solution::last_stone_weight(stones);
        assert_eq!(actual, 1);
    }
}


pub fn main() {
    println!("Hi");
}
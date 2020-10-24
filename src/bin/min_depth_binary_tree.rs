// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut to_visit: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        to_visit.push_back(root.unwrap());
        let mut depth: u32 = 1;
        let mut nodes_this_level = to_visit.len();
        loop {
            if to_visit.is_empty() {
                break;
            }
            if nodes_this_level < 1 {
                nodes_this_level = to_visit.len();
                depth = depth + 1;
                continue;
            }
            let current = to_visit.pop_front();
            nodes_this_level = nodes_this_level - 1;
            let current = current.unwrap();
            let current = current.borrow();
            if current.left.is_some() {
                let left = current.left.as_ref().unwrap();
                to_visit.push_back(left.clone());
            }
            if current.right.is_some() {
                let right = current.right.as_ref().unwrap();
                to_visit.push_back(right.clone());
            }
            if current.left.is_none() && current.right.is_none() {
                break;
            }
        }
        return depth as i32;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn simple_test() {
        assert_eq!(Solution::min_depth(None), 0);
    }

    #[test]
    pub fn simple_with_one() {
        let root = TreeNode::new(1);
        assert_eq!(Solution::min_depth(Some(Rc::from(RefCell::from(root)))), 1);
    }

    #[test]
    pub fn simple_with_two() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::from(RefCell::from(TreeNode::new(2))));

        assert_eq!(Solution::min_depth(Some(Rc::from(RefCell::from(root)))), 2);
    }
}

fn main() {
    println!("min_depth_binary_tree");
}

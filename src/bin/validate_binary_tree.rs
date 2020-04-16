use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            return (r.val as i64) < max && (r.val as i64) > min
                && Solution::dfs(&r.left, min, r.val as i64)
                && Solution::dfs(&r.right, r.val as i64, max);
        }
        return true;
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Solution::dfs(&root, std::i64::MIN, std::i64::MAX);
    }
}

pub fn main() {
    println!("validate_binary_tree");
}
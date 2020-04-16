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

impl Solution {
    pub fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let depth =
            if let Some(r) = root {
                let r = r.borrow();
                let left_depth = Solution::depth(&r.left);
                let right_depth = Solution::depth(&r.right);
                1 + std::cmp::max(left_depth, right_depth)
            } else {
                0
            };
        return depth;
    }

    #[allow(unused)]
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Solution::depth(&root);
    }
}

/**
Note:
 * it is easier to pass in root as a reference to the recursion helper method.
 * if let really simplifies the recursion helper.
 * I keep forgetting that r.left, r.right, need to be passed as a reference.
 * Is it really necessary to r = r.borrow()? yes it is. it "derefs" to a Ref.
*/

pub fn main() {
    println!("maximum_depth_of_binary_tree");
}
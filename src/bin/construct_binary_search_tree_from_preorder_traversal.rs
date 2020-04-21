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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

#[allow(unused)]
impl Solution {
    #[allow(unused)]
    pub fn from_pre_order(pre_order: &mut Vec<i32>, min:i32, max: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_order.len() < 1 {
            return None;
        }
        if pre_order[0] < min || pre_order[0] > max {
            return None;
        }
        let current = pre_order.remove(0);
        let node = TreeNode {
            val: current,
            left: Solution::from_pre_order(pre_order, min, current),
            right: Solution::from_pre_order(pre_order, current, max),
        };
        let cell = RefCell::new(node);
        let root = Rc::new(cell);
        Some(root)
    }

    #[allow(unused)]
    pub fn bst_from_pre_order(mut pre_order: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let answer =
            match pre_order.len() {
                0 => None,
                _ => Solution::from_pre_order(&mut pre_order, std::i32::MIN, std::i32::MAX)
            };
        return answer;
    }

    #[allow(unused)]
    pub fn from_post_order(mut post_order: &mut Vec<i32>, min: i32, max: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if post_order.len() < 1 {
            return None;
        }
        let last = *post_order.last().unwrap();
        if last < min || last > max {
            return None;
        }

        let current = post_order.pop().unwrap();
        let node = TreeNode {
            val: current,
            right: Solution::from_post_order(post_order, current, max),
            left: Solution::from_post_order(post_order, min, current),
        };
        let ref_cell = RefCell::new(node);
        let rc = Rc::new(ref_cell);
        Some(rc)
    }

    #[allow(unused)]
    pub fn bst_from_post_order(mut post_order: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let answer =
            match post_order.len() {
                0 => None,
                _ => Solution::from_post_order(&mut post_order, std::i32::MIN, std::i32::MAX)
            };
        return answer;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test() {
        let mut input = vec![1];
        run_test(&mut input);
    }

    #[test]
    pub fn simple_test_short() {
        let mut input = vec![2,1,3];
        run_test(&mut input);
    }

    #[test]
    pub fn simple_test_bit_longer() {
        let mut input = vec![6,10,8,7,9,12];
        run_test(&mut input);
    }

    pub fn run_test(pre_order_test_input_vec: &mut Vec<i32>) {
        let pre_order_test_input = pre_order_test_input_vec.to_vec();
        let actual_pre_order_root = Solution::bst_from_pre_order(pre_order_test_input);
        let actual_pre_order_vec = from_bst_to_pre_order(&actual_pre_order_root);
        assert_eq!(actual_pre_order_vec, pre_order_test_input_vec.to_vec());

        let post_order_test_input_vec = from_bst_to_post_order(&actual_pre_order_root);
        let actual_post_order_root = Solution::bst_from_post_order(post_order_test_input_vec.to_vec());
        let actual_post_order_vec = from_bst_to_post_order(&actual_post_order_root);
        assert_eq!(actual_post_order_vec, post_order_test_input_vec.to_vec());
    }

    pub fn pre_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            answer.push(root.val);
            tests::pre_order_traversal(&root.left, answer);
            tests::pre_order_traversal(&root.right, answer);
        }
    }

    pub fn in_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            tests::in_order_traversal(&root.left, answer);
            answer.push(root.val);
            tests::in_order_traversal(&root.right, answer);
        }
    }

    pub fn post_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            tests::post_order_traversal(&root.left, answer);
            tests::post_order_traversal(&root.right, answer);
            answer.push(root.val);
        }
    }

    pub fn from_bst_to_pre_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer:Vec<i32> = Vec::new();
        tests::pre_order_traversal(&root, &mut answer);
        return answer;
    }

    pub fn from_bst_to_in_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer:Vec<i32> = Vec::new();
        tests::in_order_traversal(&root, &mut answer);
        return answer;
    }

    pub fn from_bst_to_post_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer:Vec<i32> = Vec::new();
        tests::post_order_traversal(&root, &mut answer);
        return answer;
    }
}

pub fn main() {
    println!("construct_binary_search_tree_from_preorder_traversal");
}
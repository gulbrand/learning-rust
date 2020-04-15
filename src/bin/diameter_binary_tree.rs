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

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn diameter(root: &Option<Rc<RefCell<TreeNode>>>, max_so_far: &mut i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let l_depth = Solution::diameter(&r.left, max_so_far);
            let r_depth = Solution::diameter(&r.right, max_so_far);
            *max_so_far = std::cmp::max(
                *max_so_far,
                l_depth + r_depth + 1);
            return 1 + std::cmp::max(l_depth, r_depth);
        } else {
            0
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 1;
        Solution::diameter(&root, &mut diameter);
        return diameter - 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::TreeNode;
    use std::str::FromStr;
    use std::collections::VecDeque;

    #[test]
    pub fn simple_test() {
        run_test("1_2_#_#_#_", 1);
        run_test("1_2_#_#_3_4_#_#_5_#_#_", 3);
    }

    pub fn run_test(tree: &str, expected: i32) {
        let mut nodes: VecDeque<&str> = tree.split('_').collect();
        let root = deserialize(&mut nodes);
        let actual = Solution::diameter_of_binary_tree(root);
        assert_eq!(actual, expected);
    }

    pub fn deserialize(tree: &mut VecDeque<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        if tree.is_empty() {
            return None;
        }
        let node_str = tree.pop_front().unwrap();
        if node_str == "#" {
            return None;
        }
        let node = TreeNode {
            val: i32::from_str(node_str).unwrap(),
            left: deserialize(tree),
            right: deserialize(tree),
        };
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    pub fn test_tree_serde() {
        let root_string = "1_2_#_#_#_";
        let mut nodes: VecDeque<&str> = root_string.split('_').collect();
        let root = deserialize(&mut nodes);
            // Some(Rc::new(RefCell::new(TreeNode {val: 1, left: None, right: None})));
        let serialized = serialize(&root);
        assert_eq!(serialized, root_string);
    }

    pub fn serialize(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(r) = root {
            let r = r.borrow();
            return format!(
                "{}_{}{}",
                r.val,
                serialize(&r.left),
                serialize(&r.right));
        } else {
            return "#_".to_string();
        }
    }
}

pub fn main() {
    println!("do nothing");
}
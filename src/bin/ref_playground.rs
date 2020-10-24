use std::cell::RefCell;
use std::collections::VecDeque;
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::cmp;

    const MINIMUM_CAPACITY: usize = 1;

    #[test]
    pub fn cap_test() {
        let capacity = 0;
        let cap = cmp::max(capacity + 1, MINIMUM_CAPACITY + 1).next_power_of_two();
        assert!(cap > capacity, "capacity overflow");
        let capacity: u32 = 9;
        let cap = capacity.next_power_of_two();
        assert_eq!(cap, 16);
    }

    #[test]
    pub fn simple_test() {
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let mut root = TreeNode::new(1);
        assert_eq!(root.left, None);
        assert_eq!(root.right, None);

        root.left = Some(Rc::from(RefCell::from(TreeNode::new(2))));
        assert_ne!(root.left, None);
        queue.push_back(Some(Rc::from(RefCell::from(root))));
        assert_eq!(queue.len(), 1);

        let r = queue.pop_front();
        let r = r.unwrap();
        let r = r.unwrap();
        let r = r.borrow();
        let l = r.val;
        assert_eq!(l, 1);
        let l = &(r.left);
        // assert_ne!(r.left, None);
        {
            match (&(r.left), &None) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        println!("failed assertion");
                    }
                }
            }
        }
    }
}

pub fn create_boxed_treenode() -> Box<TreeNode> {
    Box::new(TreeNode::new(1))
}

pub fn create_refcell_treenode() -> RefCell<TreeNode> {
    RefCell::new(TreeNode::new(1))
}

fn main() {
    println!("ref_playground");
    let r1 = create_boxed_treenode();
    let r4 = create_refcell_treenode();
    let r2 = create_boxed_treenode();
    let r3 = TreeNode::new(2);

    let r1 = r1.as_ref();
    let r2 = r2.as_ref();
    let r4 = r4.as_ptr();

    println!("{:?}", r1 as *const TreeNode);
    println!("{:?}", r2 as *const TreeNode);
    println!("{:?}", &r3 as *const TreeNode);
    println!("{:?}", r4 as *const TreeNode);
}

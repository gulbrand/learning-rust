use std::borrow::Borrow;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let list_size = Solution::count_nodes(&head);
        let mut p = &head;
        for _ in 0..list_size / 2 {
            match p {
                None => return None,
                Some(ptr) => {
                    p = ptr.next.borrow();
                }
            }
        }
        let result: Option<Box<ListNode>> =
            p.clone();
        return result;
    }

    fn _count_nodes(head: &Option<Box<ListNode>>) -> i32 {
        let mut p = head;
        let mut count = 0;
        loop {
            match p {
                Some(ptr) => {
                    count += 1;
                    p = ptr.next.borrow();
                },
                None => break
            }
        }
        return count;
    }

    #[allow(dead_code)]
    pub fn count_nodes(head: &Option<Box<ListNode>>) -> i32 {
        if head.is_some() {
            return Solution::_count_nodes(head);
        }
        return 0;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Solution;
    use crate::ListNode;

    #[test]
    pub fn short_list_test() {
        let mut head_list_node = ListNode::new(0);
        let mut second_list_node = ListNode::new(1);
        let mut third_list_node = ListNode::new(2);
        third_list_node.next = None;
        second_list_node.next =
            Some(Box::from(third_list_node));
        head_list_node.next =
            Some(Box::from(second_list_node));
        let head: Option<Box<ListNode>> =
            Some(
                Box::from(
                    head_list_node
                ));
        let mid = Solution::middle_node(head);
        assert_eq!(mid.unwrap().val, 1);
    }

    #[test]
    pub fn short_list_test_even() {
        let mut head_list_node = ListNode::new(0);
        let mut second_list_node = ListNode::new(1);
        let mut third_list_node = ListNode::new(2);
        let mut fourth_list_node = ListNode::new(3);
        fourth_list_node.next = None;
        third_list_node.next =
            Some(Box::from(fourth_list_node));
        second_list_node.next =
            Some(Box::from(third_list_node));
        head_list_node.next =
            Some(Box::from(second_list_node));
        let head: Option<Box<ListNode>> =
            Some(
                Box::from(
                    head_list_node
                ));
        let mid = Solution::middle_node(head);
        assert_eq!(mid.unwrap().val, 2);
    }

    pub fn read(head: Option<Box<ListNode>>) {
        println!("{:?}", head);
        println!("{:?}", &head);
    }

    #[test]
    pub fn test_references() {
        let head: Option<Box<ListNode>> =
            Some(Box::from(ListNode::new(0)));
        read(head);
    }
}

pub fn main() {
    println!("do nothing");
}
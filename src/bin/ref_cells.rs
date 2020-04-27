use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

type NodePtr = Option<Rc<RefCell<Node>>>;


#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

struct LinkedList {
    head: NodePtr,
}

impl LinkedList {
    #[allow(unused)]
    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    #[allow(unused)]
    pub fn insert(&mut self, val: i32) {
        let new_node =
            Some(
                Rc::new(
                    RefCell::new(
                        Node {
                            val,
                            next: mem::replace(
                                &mut self.head,
                                None),
                        })));
        self.head = new_node;
    }

    #[allow(unused)]
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, None) {
            Some(h) => {
                let h = h.borrow();
                let next = &h.next;
                result = Some(h.val);
                // self.head = Some(next.unwrap());
            },
            None => {
                result = None;
            },
        };
        // let mut head = &mut self.head;
        // if head.is_some() {
        //     let reff = head.unwrap();
        //     println!("{:?}", reff);
        // }
        result
    }
}

#[cfg(test)]
pub mod tests {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    use crate::{LinkedList, Node};

    #[test]
    pub fn simple_linked_list_test() {
        let mut linked_list = LinkedList::new();
        linked_list.insert(1);
        linked_list.pop();
    }

    #[test]
    pub fn simple_test() {
        let ref_cell: RefCell<i32> = RefCell::new(0);
        let mut rc_borrow = ref_cell.borrow_mut();
        assert_eq!(0, *rc_borrow.deref());
        *rc_borrow = 1;
        assert_eq!(1, *rc_borrow);
    }

    #[test]
    pub fn simple_node_test() {
        let mut ref_cell: RefCell<Node> = RefCell::new(
            Node {
                val: 0,
                next: None
            }
        );
        let new_node: Option<Rc<RefCell<Node>>> = Some(Rc::new(RefCell::new(
            Node {
                val: 0,
                next: None,
            }
        )));
        ref_cell.get_mut().next = new_node;
    }

    #[test]
    pub fn simple_node_with_head_test() {
        let head: Option<Rc<RefCell<Node>>> = Some(Rc::new(RefCell::new(
            Node {
                val: 0,
                next: None,
            }
        )));
        let new_node: Option<Rc<RefCell<Node>>> = Some(Rc::new(RefCell::new(
            Node {
                val: 0,
                next: None,
            }
        )));

        if let Some(h) = head {
            let mut _h = h.borrow_mut();
            _h.next = new_node;
        }

    }
}

pub fn main() {
    println!("ref_cells");
}
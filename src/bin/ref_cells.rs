use std::borrow::Borrow;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub mod my_first_attempt {
    use std::cell::RefCell;
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
            let next = Some(Rc::new(RefCell::new(Node { val: 0, next: None })));
            LinkedList {
                head: next,
            }
        }

        #[allow(unused)]
        pub fn insert(&mut self, val: i32) {
            assert!(self.head.is_some());
            // ok, currently head is a sentinel node
            // if the list is empty, head.next is pointing None and its value is 0
            // to insert a new node, we want to replace head.next so it points to
            // Some(Rc(RefCell(Node)))
            // and we don't want to take ownership. So, how to access nested elements w/o taking
            // ownership. That's borrowing
            let mut borrowed_head = self.head.as_mut();

            // ok, borrowed_head should be a borrow of self.head
            // a borrow variable to an Option data/value
            let mut borrowed_head = borrowed_head.unwrap();
            let node = borrowed_head.try_borrow().unwrap();
            println!("{:?}", node);
            // this should have increased the counter on the Rc.
        }

        #[allow(unused)]
        pub fn pop(&mut self) -> Option<i32> {
            // let result;
            // match mem::replace(&mut self.head, None) {
            //     Some(h) => {
            //         let h = h.borrow();
            //         let next = &h.next;
            //         result = Some(h.val);
            //         // self.head = Some(next.unwrap());
            //     },
            //     None => {
            //         result = None;
            //     },
            // };
            // // let mut head = &mut self.head;
            // // if head.is_some() {
            // //     let reff = head.unwrap();
            // //     println!("{:?}", reff);
            // // }
            // result
            None
        }
    }


    #[cfg(test)]
    pub mod tests {
        use std::cell::RefCell;
        use std::ops::Deref;
        use std::rc::Rc;

        use crate::my_first_attempt::{LinkedList, Node};

        #[test]
        pub fn simple_ownership_test_001() {
            let mut x = Box::new(1);
            assert_eq!(*x, 1);
            *x = 2;
            assert_eq!(*x, 2);
        }

        #[test]
        pub fn simple_linked_list_test() {
            let mut linked_list = LinkedList::new();
            linked_list.insert(2);
            linked_list.insert(1);
            let popped = linked_list.pop();
            assert!(popped.is_some());
            assert_eq!(popped.unwrap(), 1);

            let popped = linked_list.pop();
            assert!(popped.is_some());
            assert_eq!(popped.unwrap(), 2);
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
                    next: None,
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
}

pub mod first {
    use std::mem;

    #[derive(Debug)]
    pub struct List {
        head: Link,
    }

    #[derive(Debug)]
    enum Link {
        Empty,
        More(Box<Node>),
    }

    #[derive(Debug)]
    struct Node {
        elem: i32,
        next: Link,
    }

    impl List {
        pub fn new() -> Self {
            List { head: Link::Empty }
        }

        pub fn push(&mut self, elem: i32) {
            let new_node = Box::new(Node {
                elem: elem,
                next: mem::replace(&mut self.head, Link::Empty),
            });
            self.head = Link::More(new_node);
        }

        pub fn pop(&mut self) -> Option<i32> {
            match mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
            }
        }
    }

    impl Drop for List {
        fn drop(&mut self) {
            println!("dropping");
            let mut cur_link = mem::replace(&mut self.head, Link::Empty);
            while let Link::More(mut boxed_node) = cur_link {
                cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            }
        }
    }

    #[cfg(test)]
    pub mod tests {
        use crate::first::List;

        #[test]
        pub fn simple_test() {
            let mut list = List::new();
            list.push(0);
            list.push(1);
            list.push(2);
            println!("{:?}", list);
            println!("{:?}", list.pop());
            println!("{:?}", list);
        }
    }
}


pub mod second {
    use std::mem;

    #[derive(Debug)]
    pub struct List<T> {
        head: Link<T>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push(&mut self, elem: T) {
            let new_node = Box::new(Node {
                elem: elem,
                next: self.head.take(),
            });
            println!("new_node -> {:p}", &new_node.elem);
            self.head = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        }

        pub fn peek(&self) -> Option<&T> {
            // self.head.as_ref().map(|node| {
            //     &node.elem
            // })
            match &self.head {
                Some(h) => Some(&h.elem),
                None => None
            }
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| {
                &mut node.elem
            })
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            println!("dropping");
            let mut cur_link = self.head.take();
            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }

    #[cfg(test)]
    pub mod tests {
        use crate::second::List;

        #[test]
        pub fn simple_test() {
            let mut list = List::new();
            println!("{:p}", &0);
            let value = 0;
            println!("{:p}", &value);
            list.push(value);
            list.push(1);
            list.push(2);
            println!("{:?}", list);
            println!("{:?}", list.pop());
            println!("{:?}", list);
            println!("{:?}", list.peek());
            println!("{:?}", list.pop());
            println!("{:?}", list.peek());
            println!("{:p}", list.peek().unwrap());
            match list.peek() {
                Some(elem) => println!("found elem, elem = {:?}", elem),
                None => println!("nothing found"),
            };

            list.peek_mut().map(|value| {
                *value = 10
            });
            assert_eq!(list.peek(), Some(&10));
            assert_eq!(list.pop(), Some(10));
        }
    }
}

pub fn main() {
    println!("ref_cells");
}
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: NodePtr<T>,
    prev: NodePtr<T>,
}

impl<T: Default+Debug> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node {
            val,
            next: None,
            prev: None
        }
    }
}

impl<T: Default+Debug> Default for Node<T> {
    fn default() -> Self {
        Node {
            val: T::default(),
            next: None,
            prev: None,
        }
    }
}


#[derive(Debug)]
struct DoublyLinkedList<T> where T: Default + Debug + Copy {
    head: NodePtr<T>,
    tail: NodePtr<T>
}

impl<T: Default+Debug+Copy> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: Some(Rc::new(RefCell::new(Node::<T>::default()))),
            tail: Some(Rc::new(RefCell::new(Node::<T>::default()))),
        }
    }

    #[allow(unused)]
    pub fn insert_at_head(&mut self, t: T) {
        let mut new_node = Node::new(t);
        let mut new_head = self.head.as_ref();
        if let Some(head) = &new_head {
            println!("it worked");
            println!("{:?}", head);
            // let head_borrow = head.borrow_mut();
            // println!("{:?}", head_borrow);
            // println!("{:?}", head_borrow.into_inner());
            //
        } else {
            println!("it didn't work");
        }
    }

}

#[cfg(test)]
pub mod doubly_linked_list_tests {
    use crate::DoublyLinkedList;

    #[test]
    pub fn first_insert() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::<i32>::new();
        list.insert_at_head(1);
    }
}
//
// struct LRUCache<K, V: Default+Debug> {
//     capacity: usize,
//     insertion_ordered_list: DoublyLinkedList<V>,
//     cache: HashMap<K, NodePtr<V>>,
// }
//
// impl<K, V: Default+Debug> LRUCache<K, V> {
//     pub fn new(capacity: usize) -> LRUCache<K, V> {
//         LRUCache {
//             capacity,
//             insertion_ordered_list: DoublyLinkedList::<V>::new(),
//             cache: HashMap::new()
//         }
//     }
//
//     pub fn get(&self, key: K) -> Option<V> {
//         None
//     }
//
//     pub fn put(&self, key: K, value: V) {
//
//     }
//
// }

pub fn main() {
    println!("linked_list");
    let _list: DoublyLinkedList<i32> = DoublyLinkedList::new();
}

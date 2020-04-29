#[derive(Debug)]
struct Foo {
    val: i32,
}

impl Foo {
    pub fn new(val: i32) -> Self {
        Foo {
            val
        }
    }
}

pub mod borrow_test_001 {
    #[derive(Debug)]
    pub struct Node {
        val: u64,
        next: Option<Box<Node>>,
        prev: Option<Box<Node>>,
    }

    pub fn borrow_test_001() {
        let head = Node {
            val: 5,
            next: None,
            prev: None,
        };
        let next = Node {
            val: 6,
            next: None,
            prev: Some(Box::new(head)),
        };
        // head.next = Some(Box::new(next));
        // println!("{:?}", &head);
        println!("{:?}", next);
    }
}

pub mod borrow_test_002 {
    #[derive(Debug)]
    pub struct Node<'a> {
        val: u64,
        next: Option<&'a Box<Node<'a>>>,
        prev: Option<&'a Box<Node<'a>>>,
    }

    pub fn borrow_test_002() {
        println!("borrow_test_002");
        let head = Node {
            val: 5,
            next: None,
            prev: None,
        };
        let head_box = Box::new(head);
        let next = Node {
            val: 6,
            next: None,
            prev: Some(&head_box),
        };
        let _next_box = Box::new(next);
        // head.next = Some(&next_box);
        // println!("{:?}", &head);
        // println!("{:?}", &next);
    }
}

pub fn main() {
    println!("ownership");
    let _foo_one = Foo::new(1); // foo_one is the owner.

    borrow_test_001::borrow_test_001();
    borrow_test_002::borrow_test_002();
}
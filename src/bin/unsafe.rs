struct Cell {
    val: i32,
    next: Option<Box<Cell>>,
}

pub fn main() {
    let cell_1 = Cell {
        val: 1,
        next: Some(Box::new(Cell { val: 2, next: None })),
    };
    unsafe {
        println!("hah, just kidding, it is safe");
    }
}

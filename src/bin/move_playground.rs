pub fn simple_move() -> String {
    let s = "String To Move".to_string();
    s
}

fn main() {
    let x = 1;
    println!("{:p}", &x);
    let y = x;
    println!("{:p}", &y);
}

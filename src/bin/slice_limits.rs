fn main() {
    println!("hi");
    let data: Vec<i32> =
        (0..1_000_000_000).collect();

    println!("got the vec");

    let a = data.as_slice();
    println!("a.len = {}", a.len());
    println!("finished");
}
#[cfg(test)]
pub mod tests {
    use std::mem;

    #[test]
    pub fn string_iterate_test() {
        println!("size of char is {}", mem::size_of::<char>());
        let s = "This is a test".to_string();
        for c in s.chars() {
            print!("{}", c);
        }
    }
}

pub fn main() {
    println!("strings_test");
}

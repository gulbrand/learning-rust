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

    #[test]
    fn for_loops_two() {
        let words: [&'static str; 3] = ["I", "love", "Rust"];
        let mut sentence: String = String::new();
        for (i, word) in words.iter().enumerate() {
            sentence.push_str(*word);
            if i < words.len() - 1 {
                sentence.push_str(" ");
            }
        }
        println!("{:?}", words.join(" "));
        // dbg!(&sentence);
        // println!("{:?}", sentence);
        // assert!(sentence == "I love Rust".to_string());
    }
}

pub fn main() {
    println!("strings_test");
}

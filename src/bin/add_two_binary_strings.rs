#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn simple_test() {
        for i in 0..4 {
            println!("i = {}, % 2 = {}, / 2 = {}",
                     i,
                     i % 2,
                     i / 2
            );
        }
    }
}

fn main() {
    println!("Hi");
}
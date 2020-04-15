#[allow(dead_code)]
fn swap_hahahahaha(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

#[allow(dead_code)]
fn swap(a: i32, b: i32) -> (i32, i32) {
    let a = b - a;
    let b = b - a;
    let a = a + b;
    (a, b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_swap_works() {
        let a: i32 = 5;
        let b: i32 = 7;
        let (a, b) = swap(a, b);
        assert_eq!(a, 7);
        assert_eq!(b, 5);
    }

    #[test]
    fn test_that_swap_works_hahahahaha() {
        let a: i32 = 5;
        let b: i32 = 7;
        let (a, b) = swap_hahahahaha(a, b);
        assert_eq!(a, 7);
        assert_eq!(b, 5);
    }
}

fn main() {
    println!("bleh");
}

fn fib_efficient(mem: &mut [usize], n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    // transform to the array index. The nth-1 fib number is stored at index i.
    let i = n - 1;
    if mem[i] != 0 {
        return mem[i];
    }
    mem[i] =
        fib_efficient(mem, n - 1)
            + fib_efficient(mem, n - 2);
    return mem[i];
}

fn fib(n: usize) -> usize {
    let mut mem = vec![0; n];
    return fib_efficient(mem.as_mut_slice(), n);
}
#[allow(dead_code)]
fn fib_inefficient(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    return fib_inefficient(n - 1) + fib(n - 2);
}

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn small_test() {
        let actual_input = 4;
        let expected = 3;
        let actual = fib(actual_input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn huge_test() {
        let actual_input = 50;
        let expected = 12586269025;
        let actual = fib(actual_input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn huge_test_innefficient() {
        let actual_input = 90;
        let expected = 2880067194370816120;
        let actual = fib_inefficient(actual_input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn huge_test_efficient() {
        let actual_input = 90;
        let expected = 2880067194370816120;
        let actual = fib(actual_input);
        assert_eq!(actual, expected);
    }
}


fn main() {
    println!("do nothing");
}
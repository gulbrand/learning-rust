use std::cmp::Ordering;
use std::convert::TryInto;

fn do_the_thing(data: &Vec<i32>) -> i32 {
    let a = data.as_slice();
    let mut bits: [i32; 32] = [0; 32];
    for i in 0..a.len() {
        let number = a[i];
        for j in 0..32 as usize {
            let bit = number >> (j as i32) & 1;
            bits[j] = (bits[j] + bit) % 3;
        }
    }

    let mut result = 0;
    let zero = 0;
    for (i, bit) in bits.iter().enumerate() {
        match bit.cmp(&zero) {
            Ordering::Equal => (),
            _ => result = result + 2_i32.pow(i as u32),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_test() {
        let data: Vec<i32> =
            [1, 2, 2, 2].to_vec();
        let expected = 1;
        let actual = do_the_thing(&data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn simple() {
        let data: Vec<i32> =
            [6, 1, 3, 3, 3, 6, 6].to_vec();
        let expected: i32 = 1;
        let actual = do_the_thing(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn another_simple() {
        let data: Vec<i32> =
            [13, 19, 13, 13].to_vec();
        let expected: i32 = 19;
        let actual = do_the_thing(&data);
        assert_eq!(expected, actual);
    }

    //    #[test]
    fn bit_test() {
        let number = 16;
        println!("{}", format!("{:b}", number));
        for i in 0..32 as i32 {
            println!("{}", format!("{:b}", 16 >> i & 1));
        }
    }
}


fn main() {
    println!("don't call this");
}
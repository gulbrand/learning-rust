pub fn digits_from_int(n: i32) -> Vec<i32> {
    let s = n.to_string();
    let mut digits: Vec<i32> = Vec::with_capacity(s.len());
    for (_, c) in s.chars().enumerate() {
        digits.push(c as i32 - '0' as i32);
    }
    digits
}


pub fn is_happy(n: i32) -> bool {
    for digit in digits_from_int(n) {
        println!("{}", digit);
    }

    return false;
}

fn main() {
    is_happy(19);
}
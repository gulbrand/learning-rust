struct Solution;

pub fn convert_to_left_shift(shifts: &Vec<Vec<i32>>, len: usize) -> i32 {
    let shift_amount: i32 =
        shifts.iter().fold(0,
                           |a, b| {
                               let converted_b = match b[0] {
                                   1 => len as i32 - (b[1] % len as i32),
                                   _ => b[1],
                               };
                               a + converted_b
                           });
    return shift_amount % len as i32;
}

impl Solution {
    #[allow(dead_code)]
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let final_shift_amount = convert_to_left_shift(&shift, s.len());
        assert!((final_shift_amount as usize) < s.len());
        assert!(final_shift_amount >= 0);
        let chars: Vec<char> = s.chars().map(|c| c).collect();

        let mut answer = String::new();
        let start = final_shift_amount as usize;
        for i in start..start + s.len() {
            let c: char = chars[i % s.len()];
            answer.push(c);
        }
        return answer;
    }
}


#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test() {
        let input = vec![vec![0, 1], vec![1, 2]];
        println!("{:?}", input);
        let actual =
            Solution::string_shift("abc".to_string(), input);
        assert_eq!(actual, "cab".to_string());
    }

    #[test]
    pub fn simple_test_1() {
        let input = vec![vec![0, 1], vec![0, 1], vec![1, 2]];
        println!("{:?}", input);
        let actual =
            Solution::string_shift("abc".to_string(), input);
        assert_eq!(actual, "abc".to_string());
    }

    #[test]
    pub fn complex_test() {
        let _input_string = "yisxjwry";

        let input = [[1, 8], [1, 4], [1, 3], [1, 6], [0, 6], [1, 4], [0, 2], [0, 1]];
        let input =
            input.iter().map(|x| vec![x[0], x[1]]).collect();
        println!("{:?}", input);
        let actual =
            Solution::string_shift("yisxjwry".to_string(), input);
        assert_eq!(actual, "yisxjwry".to_string());
    }
}

pub fn main() {
    println!("perform_string_shift");
}
use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_groups: HashMap<String, Vec<String>> = HashMap::new();
        let mut result: Vec<Vec<String>> = Vec::new();
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            println!("{} - {:?}", s, chars);
            let key: String = chars.into_iter().collect();
            anagram_groups.entry(key)
                .or_default()
                .push(s);
        }
        println!("{:?}", anagram_groups);
        for e in anagram_groups {
            result.push(e.1);
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    pub fn test_simple() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()];
        let result = Solution::group_anagrams(strs);
        println!("{:?}", result);
    }

    #[test]
    pub fn vec_test() {
        let mut v: Vec<String> = Vec::new();
        v.push("abc".to_string());
    }
}

pub fn main() {
    println!("do nothing");
}
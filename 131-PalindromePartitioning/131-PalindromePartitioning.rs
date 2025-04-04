// Last updated: 04.04.2025, 15:43:20
use std::collections::HashMap;

fn is_palindrome(substring: &[u8]) -> bool {
    substring.iter().zip(substring.iter().rev()).take(substring.len() / 2).all(|(&left, &right)| left == right)
}

fn sub_partition<'a>(substring: &'a [u8], cache: &mut HashMap<&'a [u8], Vec<Vec<String>>>) -> Vec<Vec<String>> {
    if substring.len() == 1 {
        return vec![vec![String::from_utf8_lossy(substring).into_owned()]];
    }
    if cache.contains_key(substring) {
        cache[substring].clone()
    } else {
        let mut result = Vec::new();
        for i in 1..substring.len() {
            let right = &substring[i..];
            if is_palindrome(right) {
                let mut left = sub_partition(&substring[..i], cache);
                let right = String::from_utf8_lossy(right).into_owned();
                for item in &mut left {
                    item.push(right.clone());
                }
                result.append(&mut left);
            }
        }
        if is_palindrome(substring) {
            result.push(vec![String::from_utf8_lossy(substring).into_owned()])
        }
        cache.insert(substring, result.clone());
        result
    }
}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let bytes = s.as_bytes();
        let mut cache = HashMap::new();
        sub_partition(bytes, &mut cache)
    }
}
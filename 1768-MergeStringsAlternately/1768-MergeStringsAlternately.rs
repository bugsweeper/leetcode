use std::cmp::Ordering;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1_bytes = word1.as_bytes();
        let word2_bytes = word2.as_bytes();
        let mut answer_bytes = Vec::with_capacity(word1.len() + word2.len());
        let min_len = word1.len().min(word2.len());
        for i in 0..min_len {
            answer_bytes.push(word1_bytes[i]);
            answer_bytes.push(word2_bytes[i]);
        }
        match word1.len().cmp(&word2.len()) {
            Ordering::Equal => {},
            Ordering::Greater => answer_bytes.extend_from_slice(&word1_bytes[min_len..]),
            Ordering::Less => answer_bytes.extend_from_slice(&word2_bytes[min_len..]),
        }
        unsafe{String::from_utf8_unchecked(answer_bytes)}
    }
}
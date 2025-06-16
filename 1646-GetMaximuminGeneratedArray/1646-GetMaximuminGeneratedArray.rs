// Last updated: 16.06.2025, 12:49:36
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut is_allowed = vec![false; b'z' as usize + 1];
        for byte in allowed.bytes() {
            is_allowed[byte as usize] = true;
        }
        words
            .into_iter()
            .filter(|word| word.bytes().all(|byte| is_allowed[byte as usize]))
            .count() as i32
    }
}
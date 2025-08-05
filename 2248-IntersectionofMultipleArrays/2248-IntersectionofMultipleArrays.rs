// Last updated: 05.08.2025, 15:00:43
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words
            .into_iter()
            .filter(|word| s.starts_with(word))
            .count() as i32
    }
}
// Last updated: 01.07.2025, 11:17:56
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .into_iter()
            .filter(|pattern| word.contains(pattern))
            .count() as i32
    }
}
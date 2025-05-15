// Last updated: 15.05.2025, 08:59:05
impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut prev_group = 2;
        let mut result = Vec::with_capacity(words.len());
        for (word, group) in words.into_iter().zip(groups) {
            if group != prev_group {
                result.push(word);
                prev_group = group;
            }
        }
        result
    }
}
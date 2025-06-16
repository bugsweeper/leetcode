// Last updated: 16.06.2025, 11:36:44
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut subsequence = String::with_capacity(sequence.len() + word.len() - 1);
        subsequence.push_str(&word);
        let mut repeating = 0;
        while sequence.contains(&subsequence) {
            repeating += 1;
            subsequence.push_str(&word);
        }
        repeating
    }
}
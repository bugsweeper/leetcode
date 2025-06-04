// Last updated: 04.06.2025, 10:10:50
impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let answer_len = word.len() + 1 - num_friends as usize;
        let mut max_slice = &word[..answer_len];
        for start in 1..word.len() {
            let cur_slice = &word[start..(start + answer_len).min(word.len())];
            if max_slice < cur_slice {
                max_slice = cur_slice;
            }
        }
        max_slice.into()
    }
}
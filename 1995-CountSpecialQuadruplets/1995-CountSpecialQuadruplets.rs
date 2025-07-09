// Last updated: 09.07.2025, 14:37:41
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(mut prefix_size) = word.bytes().position(|byte| byte == ch as u8) {
            // Including first occurrence
            prefix_size += 1;
            let mut result = String::with_capacity(word.len());
            result.extend(word.bytes().take(prefix_size).rev().map(|byte| byte as char));
            result.push_str(&word[prefix_size..]);
            result
        } else {
            word
        }
    }
}
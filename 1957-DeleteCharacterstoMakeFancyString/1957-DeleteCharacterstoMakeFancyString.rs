// Last updated: 30.06.2025, 14:49:01
impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut slice = s.as_str();
        for word in words {
            if slice.len() >= word.len() && word == slice[..word.len()] {
                slice = &slice[word.len()..];
                if slice.is_empty() {
                    return true;
                }
            } else {
                return false;
            }
        }
        slice.is_empty()
    }
}
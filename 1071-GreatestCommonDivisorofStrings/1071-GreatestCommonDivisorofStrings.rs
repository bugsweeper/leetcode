// Last updated: 19.05.2025, 13:51:44
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut iter = text.split_ascii_whitespace();
        let mut word1 = if let Some(word) = iter.next() {
            word
        } else {
            return Vec::new();
        };
        let mut word2 = if let Some(word) = iter.next() {
            word
        } else {
            return Vec::new();
        };
        let mut thirds = Vec::new();
        for word3 in iter {
            if word1 == first && word2 == second {
                thirds.push(word3.into());
            }
            (word1, word2) = (word2, word3)
        }
        thirds
    }
}
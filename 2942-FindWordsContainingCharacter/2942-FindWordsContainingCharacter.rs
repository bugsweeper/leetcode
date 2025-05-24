// Last updated: 24.05.2025, 09:02:34
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .into_iter()
            .enumerate()
            .filter_map(|(index, word)| {
                if word.contains(x) {
                    Some(index as i32)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
// Last updated: 08.05.2025, 12:18:20
use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut word_count = HashMap::with_capacity(120);
        for sentence in [&s1, &s2] {
            for word in sentence.split_ascii_whitespace() {
                word_count
                    .entry(word)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        word_count
            .into_iter()
            .filter_map(|(word, count)| if count == 1 { Some(word.into()) } else { None })
            .collect()
    }
}
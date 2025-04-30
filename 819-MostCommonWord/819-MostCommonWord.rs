// Last updated: 30.04.2025, 13:39:46
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned = banned
            .iter()
            .map(|banned| banned.as_bytes())
            .collect::<HashSet<_>>();
        let mut start = 0;
        let mut word_count = HashMap::with_capacity(400);
        for (index, &letter) in paragraph
            .as_bytes()
            .iter()
            .chain(std::iter::once(&b' '))
            .enumerate()
        {
            if !letter.is_ascii_alphabetic() {
                if index > start {
                    let word = paragraph[start..index].to_ascii_lowercase();
                    if !banned.contains(word.as_bytes()) {
                        word_count
                            .entry(word)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
                start = index + 1;
            }
        }
        word_count
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .unwrap()
            .0
    }
}
// Last updated: 19.06.2025, 15:14:32
use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(|symbol: char| !symbol.is_ascii_digit())
            .filter_map(|slice| {
                if slice.is_empty() {
                    None
                } else {
                    if let Some(index) = slice.find(|symbol| symbol != '0') {
                        Some(&slice[index..])
                    } else {
                        Some("0")
                    }
                }
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
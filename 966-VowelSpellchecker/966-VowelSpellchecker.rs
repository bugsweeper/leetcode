// Last updated: 14.09.2025, 14:26:51
use std::collections::{HashMap, HashSet};

const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const IS_VOWEL: [bool; ABC_LEN] = [
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false,
];

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut exactly_matches = HashSet::with_capacity(wordlist.len());
        let mut lowercased = HashMap::with_capacity(wordlist.len());
        let mut vowels_replaced = HashMap::with_capacity(wordlist.len());
        for word in &wordlist {
            let lower = word.to_ascii_lowercase();
            vowels_replaced
                .entry(
                    lower
                        .bytes()
                        .map(|byte| {
                            if IS_VOWEL[(byte - b'a') as usize] {
                                '_'
                            } else {
                                byte as char
                            }
                        })
                        .collect::<String>(),
                )
                .or_insert(word);
            lowercased.entry(lower).or_insert(word);
            exactly_matches.insert(word);
        }
        queries
            .into_iter()
            .map(|query| {
                if exactly_matches.contains(&query) {
                    query
                } else {
                    let lower = query.to_ascii_lowercase();
                    if let Some(source) = lowercased.get(&lower) {
                        (*source).clone()
                    } else if let Some(source) = vowels_replaced.get(
                        &lower
                            .bytes()
                            .map(|byte| {
                                if IS_VOWEL[(byte - b'a') as usize] {
                                    '_'
                                } else {
                                    byte as char
                                }
                            })
                            .collect::<String>(),
                    ) {
                        (*source).clone()
                    } else {
                        String::new()
                    }
                }
            })
            .collect::<Vec<_>>()
    }
}
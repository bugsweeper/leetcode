// Last updated: 14.05.2025, 14:02:18
#[inline]
fn differs_by_single_letter(first: &[u8], second: &[u8]) -> bool {
    let mut differs = false;
    for (first, second) in first.iter().zip(second) {
        if *first != *second {
            if differs {
                return false;
            }
            differs = true;
        }
    }
    differs
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_list = word_list;
        let mut end_found = false;
        for (index, word) in word_list.iter().enumerate() {
            if word == &end_word {
                end_found = true;
                word_list.swap_remove(index);
                break;
            }
        }
        if !end_found {
            return 0;
        }
        let mut words = 2;
        if differs_by_single_letter(begin_word.as_bytes(), end_word.as_bytes()) {
            return words;
        }
        let mut from = vec![begin_word];
        while !word_list.is_empty() && !from.is_empty() {
            words += 1;
            let mut to = Vec::with_capacity(word_list.len());
            for from_word in from {
                let mut pending = Vec::with_capacity(word_list.len());
                for to_word in word_list {
                    if differs_by_single_letter(from_word.as_bytes(), to_word.as_bytes()) {
                        if differs_by_single_letter(to_word.as_bytes(), end_word.as_bytes()) {
                            return words;
                        } else {
                            to.push(to_word);
                        }
                    } else {
                        pending.push(to_word);
                    }
                }
                word_list = pending;
            }
            from = to;
        }
        0
    }
}
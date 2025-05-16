// Last updated: 16.05.2025, 12:25:00
#[inline(always)]
fn single_diff(word1: &[u8], word2: &[u8]) -> bool {
    let mut has_diff = false;
    for (&byte1, &byte2) in word1.iter().zip(word2) {
        if byte1 != byte2 {
            if has_diff {
                return false;
            }
            has_diff = true;
        }
    }
    has_diff
}

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut lengths = Vec::with_capacity(words.len());
        let mut prev_indexes = Vec::with_capacity(words.len());
        lengths.push(1);
        prev_indexes.push(usize::MAX);
        let mut max_length = 1;
        let mut max_index = 0;
        for (index, (word, &group)) in words.iter().zip(&groups).enumerate().skip(1) {
            let mut length = 1;
            let mut best_prev_index = usize::MAX;
            for (prev_index, ((prev_word, &prev_group), &prev_length)) in words
                .iter()
                .zip(&groups)
                .zip(&lengths)
                .enumerate()
                .take(index)
            {
                if prev_length + 1 <= length {
                    continue;
                }
                if prev_group == group {
                    continue;
                }
                if word.len() != prev_word.len() {
                    continue;
                }
                if !single_diff(word.as_bytes(), prev_word.as_bytes()) {
                    continue;
                }
                length = prev_length + 1;
                best_prev_index = prev_index;
            }
            lengths.push(length);
            prev_indexes.push(best_prev_index);
            if max_length < length {
                max_length = length;
                max_index = index;
            }
        }
        let mut index = max_index;
        let mut indexes = Vec::with_capacity(max_length);
        while index != usize::MAX {
            indexes.push(index);
            index = prev_indexes[index];
        }
        let mut words = words;
        indexes
            .into_iter()
            .rev()
            .map(|index| std::mem::take(words.get_mut(index).unwrap()))
            .collect::<Vec<_>>()
    }
}
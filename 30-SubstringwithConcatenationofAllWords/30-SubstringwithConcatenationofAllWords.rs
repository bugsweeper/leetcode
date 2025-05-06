// Last updated: 06.05.2025, 12:03:04
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let substring_length = words.len() * word_len;
        if s.len() < substring_length {
            return vec![];
        }
        let mut word2index = HashMap::with_capacity(words.len());
        let mut goal_stats = Vec::with_capacity(words.len());
        for word in &words {
            if let Some(&index) = word2index.get(word.as_str()) {
                goal_stats[index] += 1;
            } else {
                let index = word2index.len();
                word2index.insert(word.as_str(), index);
                goal_stats.push(1);
            }
        }
        let mut checked = vec![false; s.len()];
        let mut result = Vec::with_capacity(s.len() - substring_length + 1);
        for i in 0..=s.len() - substring_length {
            if checked[i] {
                continue;
            }
            let mut current_stats = vec![0; words.len()];
            let mut index_queue = VecDeque::with_capacity(words.len());
            let mut right = i;
            while right + word_len <= s.len() {
                checked[right] = true;
                if let Some(&right_word_index) = word2index.get(&s[right..right + word_len]) {
                    index_queue.push_back(right_word_index);
                    let stats = current_stats.get_mut(right_word_index).unwrap();
                    *stats += 1;
                    if *stats > goal_stats[right_word_index] {
                        while let Some(left_word_index) = index_queue.pop_front() {
                            current_stats[left_word_index] -= 1;
                            if left_word_index == right_word_index {
                                break;
                            }
                        }
                    }
                    if index_queue.len() == words.len() {
                        result.push((right + word_len - substring_length) as i32);
                    }
                    right += word_len;
                } else {
                    break;
                }
            }
        }
        result
    }
}
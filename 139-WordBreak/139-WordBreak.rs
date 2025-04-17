// Last updated: 17.04.2025, 13:24:24
use std::collections::VecDeque;
use std::cmp::Ordering;

const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[derive(Default)]
struct Trie {
    finished: bool,
    subtries: [Option<Box<Trie>>; ABC_LEN],
}

impl Trie {
    fn add_word(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.finished = true;
        } else {
            let subtrie = self.subtries.get_mut((word[0] - b'a') as usize).unwrap();
            if subtrie.is_none() {
                *subtrie = Some(Default::default());
            }
            subtrie.as_mut().unwrap().add_word(&word[1..]);
        }
    }
    fn check_segments(
        &self,
        row: &[u8],
        start_index: usize,
        queue: &mut VecDeque<usize>,
        seen: &mut [bool],
    ) -> bool {
        if self.finished {
            match start_index.cmp(&row.len()) {
                Ordering::Equal => return true,
                Ordering::Greater => return false,
                Ordering::Less => {
                    let seen = seen.get_mut(start_index).unwrap();
                    if !*seen {
                        queue.push_back(start_index);
                        *seen = true;
                    }
                }
            }
        } else if start_index >= row.len() {
            return false;
        }
        if let Some(subtrie) = self
            .subtries
            .get((row[start_index] - b'a') as usize)
            .as_ref()
            .unwrap()
        {
            subtrie.check_segments(row, start_index + 1, queue, seen)
        } else {
            false
        }
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut trie = Trie::default();
        for word in word_dict {
            trie.add_word(word.as_bytes());
        }
        let mut queue = VecDeque::with_capacity(s.len());
        let mut seen = vec![false; s.len()];
        queue.push_back(0);
        seen[0] = true;
        while let Some(start_index) = queue.pop_front() {
            if trie.check_segments(s.as_bytes(), start_index, &mut queue, &mut seen) {
                return true;
            }
        }
        false
    }
}

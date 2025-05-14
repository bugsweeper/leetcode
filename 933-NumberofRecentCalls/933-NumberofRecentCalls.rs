// Last updated: 14.05.2025, 14:58:04
use std::cmp::Ordering;

const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut indexes = vec![0; ABC_LEN];
        for (index, &c) in order.as_bytes().iter().enumerate() {
            indexes[(c - b'a') as usize] = index;
        }
        'words: for slice in words.windows(2) {
            let [first, second] = slice else {
                unimplemented!()
            };
            for (first, second) in first.as_bytes().iter().zip(second.as_bytes()) {
                match indexes[(first - b'a') as usize].cmp(&indexes[(second - b'a') as usize]) {
                    Ordering::Greater => return false,
                    Ordering::Less => continue 'words,
                    _ => {}
                }
            }
            if first.len() > second.len() {
                return false;
            }
        }
        true
    }
}
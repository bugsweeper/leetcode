// Last updated: 13.10.2025, 16:20:13
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut prev_count = [0; ABC_LEN];
        words
            .into_iter()
            .filter(|word| {
                let mut count = [0; ABC_LEN];
                for byte in word.bytes() {
                    count[(byte - b'a') as usize] += 1;
                }
                if count == prev_count {
                    false
                } else {
                    prev_count = count;
                    true
                }
            })
            .collect()
    }
}
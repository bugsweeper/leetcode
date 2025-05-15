// Last updated: 15.05.2025, 15:23:29
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut min = [usize::MAX; ABC_LEN];
        for word in words {
            let mut frequency = [0; ABC_LEN];
            for &byte in word.as_bytes() {
                frequency[(byte - b'a') as usize] += 1;
            }
            for (min, frequency) in min.iter_mut().zip(frequency) {
                *min = (*min).min(frequency);
            }
        }
        let mut result = Vec::with_capacity(100);
        for (index, min) in min.into_iter().enumerate() {
            let character = (index as u8 + b'a') as char;
            for _ in 0..min {
                result.push(character.into());
            }
        }
        result
    }
}
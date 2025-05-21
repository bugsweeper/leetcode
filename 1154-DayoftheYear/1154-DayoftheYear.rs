// Last updated: 21.05.2025, 12:07:37
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

fn count_characters(word: &str) -> [usize; ABC_LEN] {
    let mut count = [0; ABC_LEN];
    for &c in word.as_bytes() {
        count[(c - b'a') as usize] += 1;
    }
    count
}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let chars_count = count_characters(&chars);
        let mut result = 0;
        'word: for word in words {
            let word_count = count_characters(&word);
            for (&chars_count, &word_count) in chars_count.iter().zip(word_count.iter()) {
                if chars_count < word_count {
                    continue 'word;
                }
            }
            result += word.len() as i32;
        }
        result
    }
}
// Last updated: 25.07.2025, 13:23:30
impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut words = 0;
        'word: for word in sentence.split_ascii_whitespace() {
            let mut prev_byte = b' ';
            let mut seen_hyphen = false;
            if word.is_empty() {
                continue;
            }
            for byte in word.bytes() {
                if byte.is_ascii_digit() {
                    continue 'word;
                }
                if prev_byte == b'-' && !byte.is_ascii_lowercase() {
                    continue 'word;
                }
                if byte == b'-' {
                    if seen_hyphen || !prev_byte.is_ascii_lowercase() {
                        continue 'word;
                    }
                    seen_hyphen = true;
                }
                if [b'.', b',', b'!'].contains(&prev_byte) {
                    continue 'word;
                }
                prev_byte = byte;
            }
            if prev_byte == b'-' {
                continue;
            }
            words += 1;
        }
        words
    }
}
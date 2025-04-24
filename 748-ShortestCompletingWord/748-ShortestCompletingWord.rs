// Last updated: 24.04.2025, 16:42:45
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut plate_statistic = [0; ABC_LEN];
        for byte in license_plate.as_bytes() {
            if byte.is_ascii_alphabetic() {
                plate_statistic[(byte.to_ascii_lowercase() - b'a') as usize] += 1;
            }
        }
        let mut result = String::new();
        let mut result_length = usize::MAX;
        for word in words {
            if word.len() >= result_length {
                continue;
            }
            let mut word_statistic = [0; ABC_LEN];
            for byte in word.as_bytes() {
                word_statistic[(byte - b'a') as usize] += 1;
            }
            if plate_statistic
                .iter()
                .zip(word_statistic)
                .all(|(&plate, word)| plate <= word)
            {
                result_length = word.len();
                result = word;
            }
        }
        result
    }
}
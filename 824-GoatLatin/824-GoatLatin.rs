// Last updated: 02.05.2025, 16:00:14
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const IS_VOWEL: [bool; ABC_LEN] = [
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false,
];

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut result = String::with_capacity((sentence.len() / 2).pow(2) / 2);
        for (index, word) in sentence.split_ascii_whitespace().enumerate() {
            let first = word.as_bytes()[0];
            let letter_index = (first - if first > b'Z' { b'a' } else { b'A' }) as usize;
            if IS_VOWEL[letter_index] {
                result.push_str(word);
            } else {
                result.push_str(&word[1..]);
                result.push(first as char);
            }
            result.push_str("ma");
            result.extend((0..(index + 1)).map(|_| 'a'));
            result.push(' ');
        }
        let _ = result.pop();
        result
    }
}
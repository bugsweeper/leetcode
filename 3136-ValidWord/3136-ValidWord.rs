// Last updated: 15.07.2025, 10:34:22
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const IS_VOWEL: [bool; ABC_LEN] = [
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false,
];

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let (mut has_vowel, mut has_consonant) = (false, false);
        for byte in word.bytes() {
            match byte {
                b'0'..=b'9' => {}
                b'A'..=b'Z' => {
                    if IS_VOWEL[(byte - b'A') as usize] {
                        has_vowel = true;
                    } else {
                        has_consonant = true;
                    }
                }
                b'a'..=b'z' => {
                    if IS_VOWEL[(byte - b'a') as usize] {
                        has_vowel = true;
                    } else {
                        has_consonant = true;
                    }
                }
                _ => return false,
            }
        }
        has_vowel && has_consonant
    }
}
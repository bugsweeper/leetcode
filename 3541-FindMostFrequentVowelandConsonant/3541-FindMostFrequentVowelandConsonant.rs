// Last updated: 13.09.2025, 15:10:53
const A: usize = b'a' as usize;
const Z: usize = b'z' as usize;
const ABC_LEN: usize = Z - A + 1;
const IS_VOWEL: [bool; ABC_LEN] = [
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false,
];

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut count = [0; Z + 1];
        for byte in s.bytes() {
            count[byte as usize] += 1;
        }
        let (mut max_vowels, mut max_consonants) = (0, 0);
        for (letter, &count) in count[A..=Z].iter().enumerate() {
            if IS_VOWEL[letter] {
                max_vowels = max_vowels.max(count);
            } else {
                max_consonants = max_consonants.max(count);
            }
        }
        max_vowels + max_consonants
    }
}
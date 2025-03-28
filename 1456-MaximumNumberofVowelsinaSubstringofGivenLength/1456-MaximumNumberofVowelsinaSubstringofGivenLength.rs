// Last updated: 28.03.2025, 15:14:06
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

const VOWEL: [i32; ABC_LEN] = [
    1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
];

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let uk = k as usize;
        let bytes = s.as_bytes();
        let mut vowels = bytes
            .iter()
            .take(uk)
            .map(|byte| VOWEL[(byte - b'a') as usize])
            .sum();
        if vowels == k {
            return k;
        }
        let mut max_vowels = vowels;
        for right in uk..bytes.len() {
            vowels +=
                VOWEL[(bytes[right] - b'a') as usize] - VOWEL[(bytes[right - uk] - b'a') as usize];
            if max_vowels < vowels {
                max_vowels = vowels;
                if max_vowels == k {
                    return k;
                }
            }
        }
        max_vowels
    }
}
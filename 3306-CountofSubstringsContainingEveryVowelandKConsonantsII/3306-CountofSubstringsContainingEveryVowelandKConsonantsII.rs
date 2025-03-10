const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
static IS_VOWEL: [bool; ABC_LEN] = [
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false,
];
const A: usize = b'a' as usize;

fn at_least_k(word: &[u8], k: i32) -> i64 {
    let mut vowel_count = vec![0; b'z' as usize + 1];
    let mut vowels = 0;
    let mut non_vowel_count = 0;
    let mut result = 0;
    let mut left = 0;
    for right in 0..word.len() {
        let right_char = word[right] as usize;
        if IS_VOWEL[right_char - A] {
            let count = vowel_count.get_mut(right_char).unwrap();
            if *count == 0 {
                vowels += 1;
            }
            *count += 1;
        } else {
            non_vowel_count += 1;
        }
        while vowels == 5 && non_vowel_count >= k {
            result += word.len() - right;
            let left_char = word[left] as usize;
            if IS_VOWEL[left_char - A] {
                let count = vowel_count.get_mut(left_char).unwrap();
                *count -= 1;
                if *count == 0 {
                    vowels -= 1;
                }
            } else {
                non_vowel_count -= 1;
            }
            left += 1;
        }
    }
    result as i64
}

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.as_bytes();
        at_least_k(word, k) - at_least_k(word, k + 1)
    }
}
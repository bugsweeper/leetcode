// Last updated: 25.07.2025, 14:36:03
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const IS_VOWEL: [bool; ABC_LEN] = [
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false,
];

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let (mut left_start, mut left_stop, mut vowels) = (0, 0, 0);
        let mut vowel_count = [0; ABC_LEN];
        let vowel_indexes = [b'a', b'e', b'i', b'o', b'u'].into_iter()
            .map(|vowel| (vowel - b'a') as usize)
            .collect::<Vec<_>>();
        let mut substrings = 0;
        for (right_index, byte) in word.bytes().enumerate() {
            let right_byte_index = (byte - b'a') as usize;
            if IS_VOWEL[right_byte_index] {
                let count = &mut vowel_count[right_byte_index];
                *count += 1;
                if *count == 1 {
                    vowels += 1;
                }
                if vowels < vowel_indexes.len() {
                    continue;
                }
                for (index, byte) in word.bytes().enumerate().skip(left_stop) {
                    let left_byte_index = (byte - b'a') as usize;
                    let count = &mut vowel_count[left_byte_index];
                    if *count > 1 {
                        *count -= 1;
                    } else {
                        left_stop = index;
                        break;
                    }
                }
                substrings += left_stop - left_start + 1;
            } else {
                vowels = 0;
                for vowel_index in &vowel_indexes {
                    vowel_count[*vowel_index] = 0;
                }
                left_start = right_index + 1;
                left_stop = left_start;
            }
        }
        substrings as i32
    }
}
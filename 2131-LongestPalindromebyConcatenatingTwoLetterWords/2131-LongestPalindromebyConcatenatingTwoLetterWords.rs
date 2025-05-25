// Last updated: 25.05.2025, 06:34:51
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut unused_words = vec![0; ABC_LEN * ABC_LEN];
        let mut palindrome_len = 0;
        for word in words {
            let bytes = word.as_bytes();
            let [b1, b2] = bytes[..] else {
                unimplemented!();
            };
            let (i1, i2) = ((b1 - b'a') as usize, (b2 - b'a') as usize);
            let reversed = i2 * ABC_LEN + i1;
            let reversed_count = unused_words.get_mut(reversed).unwrap();
            if *reversed_count > 0 {
                palindrome_len += 4;
                *reversed_count -= 1;
            } else {
                unused_words[i1 * ABC_LEN + i2] += 1;
            }
        }
        for symetric in unused_words.into_iter().step_by(ABC_LEN + 1) {
            if symetric > 0 {
                return palindrome_len + 2;
            }
        }
        palindrome_len
    }
}
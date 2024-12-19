impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.as_bytes();
        let mut iter = s.iter();
        let word_end = iter.rposition(|&symbol| symbol != b' ').unwrap() as i32;
        let word_start = iter
            .rposition(|&symbol| symbol == b' ')
            .map(|index| index as i32)
            .unwrap_or(-1);
        word_end - word_start
    }
}
// Last updated: 23.06.2025, 16:21:08
fn word2number(word: String) -> i32 {
    let mut number = 0;
    for byte in word.bytes() {
        number = number * 10 + (byte - b'a') as i32
    }
    number
}

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        word2number(first_word) + word2number(second_word) == word2number(target_word)
    }
}
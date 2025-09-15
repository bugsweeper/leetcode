// Last updated: 15.09.2025, 09:57:14
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut is_broken = [false; b'z' as usize + 1];
        for broken in broken_letters.bytes() {
            is_broken[broken as usize] = true;
        }
        let mut all_correct = true;
        let mut count = 0;
        for byte in text.bytes() {
            if byte == b' ' {
                if all_correct {
                    count += 1;
                } else {
                    all_correct = true;
                }
            } else {
                if is_broken[byte as usize] {
                    all_correct = false;
                }
            }
        }
        if all_correct {
            count + 1
        } else {
            count
        }
    }
}
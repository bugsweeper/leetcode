// Last updated: 03.06.2025, 15:11:04
const upper2lower: u8 = b'a' - b'A';

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        for &byte in s.as_bytes() {
            if let Some(&prev_byte) = stack.last() {
                if prev_byte == byte + upper2lower || prev_byte + upper2lower == byte {
                    stack.pop();
                } else {
                    stack.push(byte);
                }
            } else {
                stack.push(byte);
            }
        }
        String::from_utf8(stack).unwrap()
    }
}
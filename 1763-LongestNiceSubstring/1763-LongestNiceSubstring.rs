// Last updated: 19.06.2025, 15:56:41
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut base = b'a';
        for (index, byte) in s.bytes().enumerate() {
            if index & 1 == 0 {
                result.push(byte as char);
                base = byte;
            } else {
                result.push((base + (byte - b'0')) as char);
            }
        }
        result
    }
}
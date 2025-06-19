// Last updated: 19.06.2025, 14:33:32
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut prev = b'1';
        for byte in s.bytes() {
            if byte == b'1' && prev == b'0' {
                return false;
            }
            prev = byte;
        }
        true
    }
}
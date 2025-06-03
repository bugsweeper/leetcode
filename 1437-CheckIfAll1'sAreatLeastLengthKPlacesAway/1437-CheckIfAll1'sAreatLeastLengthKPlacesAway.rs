// Last updated: 03.06.2025, 12:31:02
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut prev_byte = b' ';
        let mut count = 0;
        let mut max_count = 0;
        for &byte in s.as_bytes() {
            if byte == prev_byte {
                count += 1;
            } else {
                max_count = max_count.max(count);
                count = 1;
            }
            prev_byte = byte;
        }
        max_count.max(count)
    }
}
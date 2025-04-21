// Last updated: 21.04.2025, 11:21:14
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let (mut prev_count2, mut prev_count1) = (0, 0);
        let mut prev_byte = b' ';
        let mut count = 0;
        for &byte in s.as_bytes() {
            if prev_byte == byte {
                prev_count1 += 1;
            } else {
                count += prev_count2.min(prev_count1);
                prev_count2 = prev_count1;
                prev_count1 = 1;
                prev_byte = byte;
            }
        }
        count + prev_count2.min(prev_count1)
    }
}
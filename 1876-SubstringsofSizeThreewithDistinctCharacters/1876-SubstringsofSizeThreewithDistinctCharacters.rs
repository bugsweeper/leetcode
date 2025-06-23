// Last updated: 23.06.2025, 16:16:29
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }
        let bytes = s.as_bytes();
        let (mut prev1, mut prev2) = (bytes[0], bytes[1]);
        let mut count = 0;
        for byte in s.bytes().skip(2) {
            if byte != prev1 && byte != prev2 && prev1 != prev2 {
                count += 1;
            }
            (prev1, prev2) = (prev2, byte);
        }
        count
    }
}
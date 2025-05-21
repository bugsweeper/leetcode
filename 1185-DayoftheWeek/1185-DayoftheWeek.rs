// Last updated: 21.05.2025, 16:24:11
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut balance = 0;
        let mut count = 0;
        for &byte in s.as_bytes() {
            if byte == b'L' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance == 0 {
                count += 1;
            }
        }
        count
    }
}
// Last updated: 11.08.2025, 14:05:55
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut outside = true;
        let mut count = 0;
        for byte in s.bytes() {
            match byte {
                b'|' => outside = !outside,
                b'*' => if outside {
                    count += 1
                }
                _ => {}
            }
        }
        count
    }
}
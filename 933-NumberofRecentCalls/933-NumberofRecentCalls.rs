// Last updated: 13.05.2025, 16:29:55
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let (mut low, mut high) = (0, s.len() as i32);
        let mut result = Vec::with_capacity(s.len() + 1);
        for &direction in s.as_bytes() {
            if direction == b'I' {
                result.push(low);
                low += 1;
            } else {
                result.push(high);
                high -= 1;
            }
        }
        result.push(low);
        result
    }
}
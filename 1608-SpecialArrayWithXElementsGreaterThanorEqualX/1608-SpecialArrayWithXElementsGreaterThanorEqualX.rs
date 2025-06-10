// Last updated: 10.06.2025, 11:41:32
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;
        for &byte in s.as_bytes() {
            match byte {
                b'(' => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                b')' => depth -= 1,
                _ => {}
            }
        }
        max_depth
    }
}
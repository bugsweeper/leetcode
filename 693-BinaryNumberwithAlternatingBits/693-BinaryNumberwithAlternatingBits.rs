// Last updated: 21.04.2025, 11:09:05
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut prev = n & 1;
        let mut n = n >> 1;
        while n > 0 {
            if n & 1 == prev {
                return false;
            }
            prev = n & 1;
            n >>= 1;
        }
        true
    }
}
// Last updated: 30.05.2025, 11:49:07
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mut result = 0;
        for _ in 0..32 {
            result = (result << 1) + (x & 1);
            x >>= 1;
        }
        result
    }
}
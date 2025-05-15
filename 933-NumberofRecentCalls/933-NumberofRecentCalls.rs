// Last updated: 15.05.2025, 21:52:18
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        n ^ (!(-1 << (i32::BITS - n.leading_zeros()).max(1)))
    }
}
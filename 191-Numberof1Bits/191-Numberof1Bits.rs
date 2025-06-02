// Last updated: 02.06.2025, 10:43:19
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}
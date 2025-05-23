// Last updated: 23.05.2025, 13:25:57
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low + (high & 1) + (low & 1)) / 2
    }
}
// Last updated: 11.04.2025, 09:59:50
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        n / 5 + n / 25 + n / 125 + n / 625 + n / 3125
    }
}
// Last updated: 01.08.2025, 11:43:01
impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut digit_sum = 0;
        let mut remaining = num;
        while remaining != 0 {
            digit_sum += remaining % 10;
            remaining /= 10;
        }
        (num - (digit_sum & 1)) >> 1
    }
}
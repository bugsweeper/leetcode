// Last updated: 30.05.2025, 14:22:16
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            0
        } else {
            (num.ilog2() + num.count_ones()) as i32
        }
    }
}
// Last updated: 02.06.2025, 13:28:01
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let min_sum = nums.into_iter().fold((0, 0), |(min_sum, mut sum), num| {
            sum += num;
            (min_sum.min(sum), sum)
        }).0;
        0.max(-min_sum) + 1
    }
}
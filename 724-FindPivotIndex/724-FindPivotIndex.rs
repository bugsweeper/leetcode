// Last updated: 24.04.2025, 12:56:23
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (index, num) in nums.into_iter().enumerate() {
            if (left_sum << 1) + num == sum {
                return index as i32;
            }
            left_sum += num;
        }
        -1
    }
}
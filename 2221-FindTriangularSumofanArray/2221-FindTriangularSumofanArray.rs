// Last updated: 30.09.2025, 15:12:28
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        for max_index in (1..nums.len()).rev() {
            let mut prev_num = nums[max_index];
            for num in nums.iter_mut().take(max_index).rev() {
                (prev_num, *num) = (*num, (prev_num + *num) % 10);
            }
        }
        nums[0]
    }
}
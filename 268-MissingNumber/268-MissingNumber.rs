impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let expected_sum = nums.len() * (nums.len() + 1) / 2;
        expected_sum as i32 - nums.into_iter().sum::<i32>()
    }
}

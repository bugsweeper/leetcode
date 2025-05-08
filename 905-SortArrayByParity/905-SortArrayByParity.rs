// Last updated: 08.05.2025, 15:35:16
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by_key(|&num| num % 2);
        nums
    }
}
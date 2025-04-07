// Last updated: 07.04.2025, 14:57:45
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len() - k as usize]
    }
}
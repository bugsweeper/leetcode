// Last updated: 27.04.2025, 06:35:48
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.windows(3).filter(|nums| nums[1] == ((nums[0] + nums[2]) << 1)).count() as i32
    }
}
// Last updated: 06.05.2025, 09:44:47
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        for &num in &nums {
            ans.push(nums[num as usize]);
        }
        ans
    }
}
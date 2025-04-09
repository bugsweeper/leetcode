// Last updated: 09.04.2025, 11:43:04
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.into_iter().step_by(2).sum()
    }
}
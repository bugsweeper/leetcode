// Last updated: 08.05.2025, 15:42:56
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        ((nums.iter().copied().max().unwrap() - nums.iter().copied().min().unwrap()) - 2 * k).max(0)
    }
}
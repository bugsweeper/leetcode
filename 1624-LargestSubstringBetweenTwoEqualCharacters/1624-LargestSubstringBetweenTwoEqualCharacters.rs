// Last updated: 12.06.2025, 16:05:31
const LIMIT: i32 = 100;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = [0; ((LIMIT as usize) << 1) + 1];
        for &num in &nums {
            freq[(num + LIMIT) as usize] += 1;
        }
        let mut nums = nums;
        nums.sort_unstable_by_key(|num| {
            (freq[(num + LIMIT) as usize], -num)
        });
        nums
    }
}
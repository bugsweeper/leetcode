// Last updated: 30.05.2025, 16:11:27
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        let mut nums = nums;
        for num in &mut nums {
            *num = sorted.partition_point(|sorted| sorted < num) as i32;
        }
        nums
    }
}
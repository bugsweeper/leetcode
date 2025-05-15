// Last updated: 15.05.2025, 21:02:24
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let negative_count = nums.partition_point(|&num| num < 0);
        let mut k = k as usize;
        if k > nums.len() {
            k = if nums.len() & 1 == k & 1 { nums.len() } else { nums.len() - 1 };
        }
        let operations = if negative_count >= k {
            k
        } else if (k - negative_count) & 1 == 0 {
            negative_count
        } else if negative_count > 0 && nums[negative_count] > -nums[negative_count - 1] {
            negative_count - 1
        } else {
            negative_count + 1
        };
        nums.iter().skip(operations).sum::<i32>() - nums.iter().take(operations).sum::<i32>()
    }
}
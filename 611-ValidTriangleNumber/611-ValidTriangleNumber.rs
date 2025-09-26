// Last updated: 26.09.2025, 08:36:27
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut count = 0;
        for (index1, num1) in nums.iter().enumerate().take(nums.len() - 2) {
            for (index2, num2) in nums.iter().enumerate().skip(index1 + 1).take(nums.len() - index1 - 1) {
                count += nums[index2 + 1..].partition_point(|&num3| num3 < num1 + num2);
            }
        }
        count as i32
    }
}
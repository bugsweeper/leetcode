// Last updated: 14.04.2025, 08:52:01
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            nums[0]
        } else {
            let mut prev2 = nums[0];
            let mut prev1 = prev2.max(nums[1]);
            for num in nums.into_iter().skip(2) {
                (prev2, prev1) = (prev1, prev1.max(num + prev2));
            }
            prev1
        }
    }
}
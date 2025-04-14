// Last updated: 14.04.2025, 08:46:11
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            _ => {
                let mut max_amount: Vec<i32> = Vec::with_capacity(nums.len());
                max_amount.push(nums[0]);
                max_amount.push(nums[0].max(nums[1]));
                for (index, num) in nums.into_iter().enumerate().skip(2) {
                    let current_amount = max_amount[index - 1].max(num + max_amount[index - 2]);
                    max_amount.push(current_amount);
                }
                *max_amount.last().unwrap()
            }
        }
    }
}
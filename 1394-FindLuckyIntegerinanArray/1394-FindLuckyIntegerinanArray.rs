// Last updated: 02.06.2025, 13:20:19
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let goal_sum = nums.iter().sum::<i32>() / 2;
        let mut sum = 0;
        let mut subsequence = Vec::with_capacity(nums.len() / 2 + 1);
        for num in nums.into_iter().rev() {
            subsequence.push(num);
            sum += num;
            if sum > goal_sum {
                break;
            }
        }
        subsequence
    }
}
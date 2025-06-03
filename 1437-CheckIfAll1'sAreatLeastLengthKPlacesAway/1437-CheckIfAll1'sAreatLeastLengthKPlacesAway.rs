// Last updated: 03.06.2025, 12:49:51
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut nums = nums;
        for num in &mut nums {
            sum += *num;
            *num = sum;
        }
        nums
    }
}
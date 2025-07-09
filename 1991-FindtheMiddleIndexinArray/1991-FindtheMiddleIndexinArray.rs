// Last updated: 09.07.2025, 12:25:25
impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let total_sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        for (index, num) in nums.into_iter().enumerate() {
            let right_sum = total_sum - num - left_sum;
            if left_sum == right_sum {
                return index as i32;
            }
            left_sum += num;
        }
        -1
    }
}
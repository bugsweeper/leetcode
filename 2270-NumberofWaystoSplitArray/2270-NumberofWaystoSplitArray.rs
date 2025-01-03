impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let sum: i64 = nums.iter().map(|num| *num as i64).sum();
        let mut left_sum = 0;
        let mut ways = 0;
        for num in &nums[..nums.len() - 1] {
            left_sum += *num as i64;
            if left_sum >= sum - left_sum {
                ways += 1;
            }
        }
        ways
    }
}
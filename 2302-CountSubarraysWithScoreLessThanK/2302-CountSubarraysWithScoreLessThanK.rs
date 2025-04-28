// Last updated: 28.04.2025, 11:32:49
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let k = k as usize;
        let mut sum = 0;
        let mut from = 0;
        let mut count = 0;
        for (index, &num) in nums.iter().enumerate() {
            sum += num as usize;
            while from <= index && sum * (index - from + 1) >= k {
                sum -= nums[from] as usize;
                from += 1;
            }
            count += index + 1 - from;
        }
        count as i64
    }
}
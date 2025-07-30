// Last updated: 30.07.2025, 10:09:59
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let (mut len, mut max_len) = (0, 0);
        for num in nums {
            if num == max {
                len += 1;
                max_len = max_len.max(len);
            } else {
                len = 0;
            }
        }
        max_len
    }
}
// Last updated: 21.04.2025, 10:18:09
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut prev = i32::MIN;
        let mut max_len = 0;
        let mut len = 0;
        for num in nums {
            if num > prev {
                len += 1;
            } else {
                max_len = max_len.max(len);
                len = 1;
            }
            prev = num;
        }
        max_len.max(len)
    }
}
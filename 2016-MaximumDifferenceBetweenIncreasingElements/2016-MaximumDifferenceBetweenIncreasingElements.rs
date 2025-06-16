// Last updated: 16.06.2025, 08:43:42
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut max_diff = -1;
        let mut min_num = i32::MAX;
        for num in nums {
            if num > min_num {
                max_diff = max_diff.max(num - min_num);
            }
            min_num = min_num.min(num);
        }
        max_diff
    }
}
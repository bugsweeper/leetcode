impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_ones = 0;
        let mut current_ones = 0;
        for num in nums {
            if num == 1 {
                current_ones += 1;
            } else {
                max_ones = max_ones.max(current_ones);
                current_ones = 0;
            }
        }
        max_ones.max(current_ones)
    }
}
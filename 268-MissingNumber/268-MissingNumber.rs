impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let nums: std::collections::HashSet<_> = nums.into_iter().collect();
        for num in 0..nums.len() as i32 {
            if !nums.contains(&num) {
                return num;
            }
        }
        nums.len() as i32
    }
}
// Last updated: 22.05.2025, 14:31:33
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let candidates = [*arr.first().unwrap(), arr[arr.len() / 4], arr[arr.len() / 2], arr[3 * arr.len() / 4]];
        let mut max_count = 0;
        let mut goal = i32::MAX;
        for candidate in candidates {
            let count = arr.partition_point(|&num| num <= candidate) - arr.partition_point(|&num| num < candidate);
            if count > max_count {
                goal = candidate;
                max_count = count;
            }
        }
        goal
    }
}
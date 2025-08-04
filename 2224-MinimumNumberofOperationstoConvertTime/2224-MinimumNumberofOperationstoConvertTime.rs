// Last updated: 04.08.2025, 16:24:35
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .min_by_key(|&num| (num.abs() << 1) + if num > 0 { 0 } else { 1 })
            .unwrap()
    }
}
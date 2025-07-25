// Last updated: 25.07.2025, 13:40:42
impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (index, num) in nums.into_iter().enumerate() {
            if index % 10 == num as usize {
                return index as i32;
            }
        }
        -1
    }
}
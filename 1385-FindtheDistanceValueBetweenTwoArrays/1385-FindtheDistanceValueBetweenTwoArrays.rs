// Last updated: 02.06.2025, 12:36:44
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        for (num, index) in nums.into_iter().zip(index.into_iter()) {
            result.insert(index as usize, num);
        }
        result
    }
}
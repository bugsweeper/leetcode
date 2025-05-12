// Last updated: 12.05.2025, 13:34:42
impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut odd_index = 1;
        let mut even_index = 0;
        while odd_index < nums.len() && even_index < nums.len() {
            if let Some(shift) = nums[odd_index..].iter().step_by(2).copied().position(|num| num % 2 == 0) {
                odd_index += shift << 1;
            } else {
                break;
            }
            if let Some(shift) = nums[even_index..].iter().step_by(2).copied().position(|num| num % 2 == 1) {
                even_index += shift << 1;
            } else {
                break;
            }
            nums.swap(odd_index, even_index);
            odd_index += 2;
            even_index += 2;
        }
        nums
    }
}
// Last updated: 18.06.2025, 09:20:21
impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = Vec::with_capacity(nums.len() / 3);
        for slice in nums.chunks(3) {
            if slice[2] - slice[0] > k {
                return vec![];
            }
            result.push(slice.into())
        }
        result
    }
}
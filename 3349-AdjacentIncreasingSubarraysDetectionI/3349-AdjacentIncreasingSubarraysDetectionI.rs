// Last updated: 14.10.2025, 19:42:54
impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_size = 0;
        let k = k as usize;
        let k2 = k << 1;
        for chunk in nums.chunk_by(|a, b| a < b) {
            if chunk.len() >= k2 || chunk.len() >= k && prev_size >= k {
                return true;
            }
            prev_size = chunk.len();
        }
        false
    }
}
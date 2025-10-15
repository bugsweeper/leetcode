// Last updated: 15.10.2025, 09:48:29
impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut prev_size = 0;
        let mut max_k = 0;
        for chunk in nums.chunk_by(|a, b| a < b) {
            max_k = max_k.max(chunk.len() >> 1).max(chunk.len().min(prev_size));
            prev_size = chunk.len();
        }
        max_k as i32
    }
}
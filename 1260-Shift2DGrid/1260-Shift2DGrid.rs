// Last updated: 22.05.2025, 15:38:39
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len() / 2);
        for slice in nums.chunks(2) {
            let &[freq, val] = slice else {
                unimplemented!();
            };
            result.extend(std::iter::repeat_n(val, freq as usize));
        }
        result
    }
}
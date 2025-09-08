// Last updated: 08.09.2025, 16:26:43
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; 101];
        for num in nums {
            seen[num as usize] = true;
        }
        seen.into_iter().skip(1).filter(|&seen| seen).count() as i32
    }
}
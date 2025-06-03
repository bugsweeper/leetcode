// Last updated: 03.06.2025, 12:47:55
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let middle = nums.len() / 2;
        for (&x, &y) in nums[..middle].iter().zip(&nums[middle..]) {
            result.push(x);
            result.push(y);
        }
        result
    }
}
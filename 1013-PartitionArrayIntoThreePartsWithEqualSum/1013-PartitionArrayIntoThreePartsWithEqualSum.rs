// Last updated: 15.05.2025, 23:29:40
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut number = 0;
        let mut result = Vec::with_capacity(nums.len());
        for num in nums {
            number = ((number << 1) + num) % 5;
            result.push(number == 0);
        }
        result
    }
}
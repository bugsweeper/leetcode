// Last updated: 01.08.2025, 10:21:02
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut value = original;
        while nums.binary_search(&value).is_ok() {
            value <<= 1;
        }
        value
    }
}
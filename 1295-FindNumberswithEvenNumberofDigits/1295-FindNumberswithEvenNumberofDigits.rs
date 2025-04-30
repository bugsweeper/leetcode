// Last updated: 30.04.2025, 08:26:13
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|num| num.ilog10() % 2 == 1).count() as i32
    }
}
// Last updated: 27.06.2025, 11:30:20
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        nums.sort_by_key(|num| if *num == 0 {1} else {0})
    }
}
// Last updated: 11.08.2025, 13:27:50
fn min_max_game(nums: &[i32], bit: i32) -> i32 {
    if nums.len() == 1 {
        return nums[0]
    }
    let half = nums.len() >> 1;
    let left = min_max_game(&nums[..half], 0);
    let right = min_max_game(&nums[half..], 1);
    if bit == 0 {
        left.min(right)
    } else {
        left.max(right)
    }
}

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        min_max_game(&nums, 0)
    }
}
// Last updated: 03.06.2025, 12:42:23
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let (max_index, max) = nums.iter().copied().enumerate().max_by_key(|&(index, value)| value).unwrap();
        let max2 = nums.iter().copied().enumerate().filter_map(|(index, value)| if index == max_index {None} else {Some(value)}).max().unwrap();
        (max - 1) * (max2 - 1)
    }
}
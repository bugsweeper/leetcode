// Last updated: 03.06.2025, 14:09:09
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];
        for num in nums {
            count[num as usize] += 1;
        }
        count.into_iter().filter_map(|count| if count > 1 { Some(count * (count - 1) / 2) } else { None }).sum()
    }
}
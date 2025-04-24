// Last updated: 24.04.2025, 15:18:37
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (max, max_index) = nums
            .iter()
            .enumerate()
            .map(|(index, num)| (*num, index))
            .max()
            .unwrap();
        if nums
            .into_iter()
            .enumerate()
            .filter_map(|(index, num)| if index == max_index { None } else { Some(num) })
            .all(|num| max >= num << 1)
        {
            max_index as i32
        } else {
            -1
        }
    }
}
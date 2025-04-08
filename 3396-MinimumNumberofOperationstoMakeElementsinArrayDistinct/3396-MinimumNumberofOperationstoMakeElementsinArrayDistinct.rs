// Last updated: 08.04.2025, 11:35:46
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut prev_index = [None; 101];
        let mut remove_count = 0;
        for (index, num) in nums.into_iter().enumerate() {
            let prev_index = prev_index.get_mut(num as usize).unwrap();
            if let Some(prev_index) = *prev_index {
                remove_count = remove_count.max(prev_index + 1);
            }
            *prev_index = Some(index);
        }
        remove_count.div_ceil(3) as i32
    }
}
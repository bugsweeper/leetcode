use std::cmp::Ordering;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut right_index = height.len() - 1;
        let mut max_area = i32::MIN;
        while left_index < right_index {
            max_area = max_area.max(
                ((right_index - left_index) as i32) * height[right_index].min(height[left_index]),
            );
            match height[left_index].cmp(&height[right_index]) {
                Ordering::Less => left_index += 1,
                Ordering::Equal => {
                    left_index += 1;
                    right_index -= 1;
                }
                Ordering::Greater => right_index -= 1,
            }
        }
        max_area
    }
}
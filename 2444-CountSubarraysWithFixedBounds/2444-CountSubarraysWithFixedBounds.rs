// Last updated: 26.04.2025, 14:51:23
use std::cmp::Ordering;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut from = 0;
        let mut last_min_index = usize::MAX;
        let mut last_max_index = usize::MAX;
        let mut count = 0;
        for (index, num) in nums.into_iter().enumerate() {
            match num.cmp(&max_k) {
                Ordering::Greater => {
                    from = index + 1;
                    last_min_index = usize::MAX;
                    last_max_index = usize::MAX;
                    continue;
                }
                Ordering::Equal => {
                    last_max_index = index;
                }
                _ => {}
            }
            match num.cmp(&min_k) {
                Ordering::Less => {
                    from = index + 1;
                    last_min_index = usize::MAX;
                    last_max_index = usize::MAX;
                    continue;
                }
                Ordering::Equal => {
                    last_min_index = index;
                }
                _ => {}
            }
            if last_min_index == usize::MAX || last_max_index == usize::MAX {
                continue;
            }
            count += last_min_index.min(last_max_index) - from + 1;

        }
        count as i64
    }
}
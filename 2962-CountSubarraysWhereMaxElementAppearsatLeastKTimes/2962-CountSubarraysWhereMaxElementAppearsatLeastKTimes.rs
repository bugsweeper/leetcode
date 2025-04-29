// Last updated: 29.04.2025, 10:46:38
use std::collections::VecDeque;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let max = nums.iter().copied().max().unwrap();
        let mut max_indexes = VecDeque::with_capacity(k);
        let mut count = 0;
        for (index, num) in nums.into_iter().enumerate() {
            if num == max {
                if max_indexes.len() == k {
                    max_indexes.pop_front();
                }
                max_indexes.push_back(index);
            }
            if max_indexes.len() == k {
                count += *max_indexes.front().unwrap() as i64 + 1;
            }
        }
        count
    }
}
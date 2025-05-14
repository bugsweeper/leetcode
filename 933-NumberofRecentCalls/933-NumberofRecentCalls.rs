// Last updated: 14.05.2025, 16:01:24
use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; 10000];
        for num in nums {
            let seen = seen.get_mut(num as usize).unwrap();
            if *seen {
                return num;
            }
            *seen = true;
        }
        0
    }
}
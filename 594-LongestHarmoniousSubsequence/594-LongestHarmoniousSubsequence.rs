// Last updated: 11.04.2025, 15:17:05
use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut lhs: HashMap<i32, (i32, i32)> = HashMap::with_capacity(nums.len() * 3);
        for num in nums {
            lhs.entry(num).or_default().0 += 1;
            lhs.entry(num - 1).or_default().1 += 1;
        }
        lhs.into_iter()
            .map(|(_, (lower_count, higher_count))| {
                if lower_count == 0 || higher_count == 0 {
                    0
                } else {
                    lower_count + higher_count
                }
            })
            .max()
            .unwrap()
    }
}
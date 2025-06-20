// Last updated: 20.06.2025, 16:08:33
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum_count = HashMap::with_capacity(nums.len());
        sum_count.insert(0, 1);
        let mut sum = 0;
        let mut count = 0;
        for num in nums {
            sum += num;
            let prefix = sum - k;
            count += *sum_count.get(&prefix).unwrap_or(&0);
            sum_count.entry(sum).and_modify(|count| *count += 1).or_insert(1);
        }
        count
    }
}
// Last updated: 01.08.2025, 11:52:38
use std::collections::HashMap;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count = HashMap::with_capacity(nums.len() >> 1);
        let mut prev = 0;
        for num in nums {
            if prev == key {
                count.entry(num).and_modify(|count| *count += 1).or_insert(1);
            }
            prev = num;
        }
        let (mut answer, mut max_count) = (0, 0);
        for (num, count) in count {
            if count > max_count {
                answer = num;
                max_count = count;
            }
        }
        answer
    }
}
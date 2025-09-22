// Last updated: 22.09.2025, 05:51:11
use std::cmp::Ordering;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 101];
        for num in nums {
            count[num as usize] += 1;
        }
        let mut max_count = 0;
        let mut sum = 0;
        for count in count {
            match count.cmp(&max_count) {
                Ordering::Greater => {
                    max_count = count;
                    sum = count;
                }
                Ordering::Equal => sum += count,
                _ => {}
            }
        }
        sum
    }
}
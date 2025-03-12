use std::cmp::Ordering;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        // The task is to find where negative numbers end and positive start
        // It could be found by slice::partition_point, but it would make
        // two independent binary searches, but I want to cache search points
        let mut non_negative_range_start = 0;
        let mut non_negative_range_len = nums.len();
        let mut positive_range_start = 0;
        let mut positive_range_len = nums.len();
        while non_negative_range_len > 1 {
            let half = non_negative_range_len / 2;
            let middle = non_negative_range_start + half;
            match nums[middle].cmp(&0) {
                Ordering::Equal => {
                    positive_range_start = positive_range_start.max(middle);
                }
                Ordering::Greater => {
                    positive_range_len = middle - positive_range_start;
                }
                Ordering::Less => {
                    non_negative_range_start = middle;
                    positive_range_start = positive_range_start.max(middle);
                }
            }
            non_negative_range_len -= half;
        }
        if nums[non_negative_range_start] < 0 {
            non_negative_range_start += 1;
        }
        positive_range_len = positive_range_len.min(nums.len() - positive_range_start);
        while positive_range_len > 1 {
            let half = positive_range_len / 2;
            let middle = positive_range_start + half;
            if nums[middle] <= 0 {
                positive_range_start = middle;
            }
            positive_range_len -= half;
        }
        if nums[positive_range_start] <= 0 {
            positive_range_start += 1;
        }
        (non_negative_range_start).max(nums.len() - positive_range_start) as i32
    }
}
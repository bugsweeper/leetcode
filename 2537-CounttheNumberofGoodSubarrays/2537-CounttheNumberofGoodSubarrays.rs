// Last updated: 16.04.2025, 14:07:10
use std::collections::{HashMap, hash_map::Entry::Occupied};

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut left = 0;
        let mut right = 0;
        let mut statistic = HashMap::with_capacity(nums.len());
        let mut pairs = 0;
        let mut count = 0;
        while right < nums.len() {
            let added_num: &mut i32 = statistic.entry(nums[right]).or_default();
            if *added_num > 0 {
                pairs += *added_num;
            }
            *added_num += 1;
            while pairs >= k {
                count += (nums.len() - right) as i64;
                let Occupied(mut count) = statistic.entry(nums[left]) else {
                    unimplemented!()
                };
                if *count.get() == 1 {
                    count.remove();
                } else {
                    *count.get_mut() -= 1;
                    pairs -= *count.get();
                }
                left += 1;
            }
            right += 1;
        }
        count
    }
}


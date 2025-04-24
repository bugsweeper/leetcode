// Last updated: 24.04.2025, 10:33:36
use std::collections::{HashMap, hash_map::Entry::Occupied};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut items = HashMap::with_capacity(nums.len());
        for &num in &nums {
            items
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let goal_items = items.len();
        items.clear();
        let mut left_iter = nums.iter();
        let mut subarrays_count = 0;
        for (right_index, &right_num) in nums.iter().enumerate() {
            items
                .entry(right_num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            if items.len() == goal_items {
                subarrays_count += nums.len() - right_index;
                loop {
                    let left_num = *left_iter.next().unwrap();
                    let Occupied(mut left_entry) = items.entry(left_num) else {
                        unimplemented!();
                    };
                    match left_entry.get_mut() {
                        &mut 1 => {
                            left_entry.remove();
                            break;
                        }
                        other => {
                            *other -= 1;
                            subarrays_count += nums.len() - right_index;
                        }
                    }
                }
            }
        }
        subarrays_count as i32
    }
}
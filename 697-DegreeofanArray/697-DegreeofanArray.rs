// Last updated: 21.04.2025, 11:39:12
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut statistics = HashMap::with_capacity(nums.len());
        let mut max_count = 0;
        for (index, num) in nums.into_iter().enumerate() {
            let s = statistics.entry(num).or_insert((0, index, index));
            s.0 += 1;
            s.2 = index;
            max_count = max_count.max(s.0);
        }
        statistics
            .into_values()
            .filter_map(|(count, start, end)| if count == max_count { Some(end - start + 1) } else { None })
            .min()
            .unwrap() as i32
    }
}
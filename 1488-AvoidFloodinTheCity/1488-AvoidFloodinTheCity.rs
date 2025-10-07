// Last updated: 07.10.2025, 14:06:23
use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut lakes = HashMap::with_capacity(rains.len());
        let mut unused_zeros = BTreeSet::new();
        let mut answer = vec![-1; rains.len()];
        for (index, &rain) in rains.iter().enumerate() {
            if rain == 0 {
                unused_zeros.insert(index);
                answer[index] = 1;
            } else {
                if let Some(prev_index) = lakes.insert(rain, index) {
                    if let Some(&next_zero_index) = unused_zeros.range(prev_index..).next() {
                        unused_zeros.remove(&next_zero_index);
                        answer[next_zero_index] = rain;
                    } else {
                        return Vec::new();
                    }
                }
            }
        }
        answer
    }
}
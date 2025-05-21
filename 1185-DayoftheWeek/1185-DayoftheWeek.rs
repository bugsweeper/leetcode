// Last updated: 21.05.2025, 14:54:25
use std::cmp::Ordering;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();
        let mut min_difference = i64::MAX;
        let mut difference_count = 0;
        for slice in arr.windows(2) {
            let &[a, b] = slice else {
                unimplemented!();
            };
            let difference = b as i64 - a as i64;
            match difference.cmp(&min_difference) {
                Ordering::Less => {
                    min_difference = difference;
                    difference_count = 1;
                }
                Ordering::Equal => difference_count += 1,
                _ => {}
            }
        }
        let mut result = Vec::with_capacity(difference_count);
        result.extend(arr.windows(2).filter_map(|slice| {
            let &[a, b] = slice else {
                unimplemented!();
            };
            let difference = b as i64 - a as i64;
            if difference == min_difference {
                Some(vec![a, b])
            } else {
                None
            }
        }));
        result
    }
}
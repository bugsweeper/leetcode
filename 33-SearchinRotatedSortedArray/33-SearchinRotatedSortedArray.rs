use std::cmp::Ordering;

const MAX_VALUE: i32 = 10_000;
const MIN_VALUE: i32 = -10_1000;
const RANGE_OVERFLOW: i32 = MAX_VALUE - MIN_VALUE + 1;


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let first = unsafe { nums.get_unchecked(0) };
        let target = match target.cmp(first) {
            Ordering::Equal => return 0,
            Ordering::Greater => target - RANGE_OVERFLOW,
            Ordering::Less => target,
        };
        if let Ok(index) = nums.binary_search_by_key(&target, |num| if num < first { *num } else { num - RANGE_OVERFLOW }) {
            index as i32
        } else {
            -1
        }
    }
}
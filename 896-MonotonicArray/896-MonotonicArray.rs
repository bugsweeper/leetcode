// Last updated: 25.04.2025, 16:02:46
use std::cmp::Ordering;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut iter = nums.into_iter();
        let first = iter.next().unwrap();
        let mut prev = first;
        let mut ordering = Ordering::Equal;
        while let Some(next) = iter.next() {
            match first.cmp(&next) {
                Ordering::Equal => {},
                other => {
                    ordering = other;
                    prev = next;
                    break;
                }
            }
        }
        if ordering == Ordering::Equal {
            return true;
        }
        for num in iter {
            match prev.cmp(&num) {
                Ordering::Equal => {},
                other => {
                    if ordering != other {
                        return false;
                    }
                }
            }
            prev = num;
        }
        true
    }
}
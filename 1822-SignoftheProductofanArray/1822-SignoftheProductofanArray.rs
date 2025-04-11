// Last updated: 11.04.2025, 10:49:17
use std::cmp::Ordering;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut answer = 1;
        for num in nums {
            match num.cmp(&0) {
                Ordering::Equal => return 0,
                Ordering::Less => answer = -answer,
                _ => {},
            }
        }
        answer
    }
}
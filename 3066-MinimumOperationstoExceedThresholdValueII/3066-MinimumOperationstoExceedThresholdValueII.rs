use std::cmp::Reverse;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut smaller = std::collections::BinaryHeap::with_capacity(nums.len());
        for num in nums {
            smaller.push(Reverse(num as i64));
        }
        let mut count = 0;
        loop {
            let Reverse(x) = smaller.pop().unwrap();
            if x >= k {
                return count;
            }
            let Reverse(y) = smaller.pop().unwrap();
            count += 1;
            smaller.push(Reverse(2 * x.min(y) + x.max(y)));
        }
    }
}
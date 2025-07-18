// Last updated: 18.07.2025, 11:04:00
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;
        let mut prefix_min = Vec::with_capacity(n + 1);
        // BinaryHeap is max-heap, but if we push any number but pop its max, then there would be top min values
        let mut left_mins = BinaryHeap::with_capacity(n + 1);
        let mut sum = 0;
        for &num in nums.iter().take(n) {
            sum += num as i64;
            left_mins.push(num);
        }
        prefix_min.push(sum);
        for &num in nums.iter().skip(n).take(n) {
            left_mins.push(num);
            let max = left_mins.pop().unwrap();
            sum += (num - max) as i64;
            prefix_min.push(sum);
        }
        let mut suffix_max = Vec::with_capacity(n + 1);
        // Prevent reallocating by taking prev max-heap and clearing it
        let mut right_max = left_mins;
        right_max.clear();
        sum = 0;
        for &num in nums.iter().rev().take(n) {
            sum += num as i64;
            // push negative value into max-heap to pop min absolute values at each step for keeping top max values
            right_max.push(-num);
        }
        suffix_max.push(sum);
        for &num in nums.iter().rev().skip(n).take(n) {
            right_max.push(-num);
            let negative_min = right_max.pop().unwrap();
            sum += (num + negative_min) as i64;
            suffix_max.push(sum);
        }
        prefix_min
            .into_iter()
            .zip(suffix_max.into_iter().rev())
            .map(|(left_sum, right_sum)| left_sum - right_sum)
            .min()
            .unwrap()
    }
}
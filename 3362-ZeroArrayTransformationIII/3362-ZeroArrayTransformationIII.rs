// Last updated: 22.05.2025, 12:00:33
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut queries = queries.into_iter().map(|range| (range[0] as usize, range[1] as usize)).collect::<Vec<_>>();
        queries.sort_unstable_by_key(|(_, right)| *right);
        let mut heap = BinaryHeap::with_capacity(queries.len());
        // contains applied queries to the left
        let mut prefix_sum = vec![0; nums.len()];
        // contains applied queries to the right
        let mut sum = 0;
        for (index, mut num) in nums.into_iter().enumerate().rev() {
            num -= sum;
            // prefix_sum[index] contains finished queries here
            sum += prefix_sum[index];
            if num <= 0 {
                continue;
            }
            let first_query = queries.partition_point(|(_, right)| *right < index);
            if queries.len() - first_query + heap.len() < num as usize {
                return -1;
            }
            for (left, _) in queries.drain(first_query..) {
                heap.push(Reverse(left));
            }
            for _ in 0..num {
                let Reverse(left) = heap.pop().unwrap();
                if left > index {
                    return -1;
                }
                if left < index {
                    prefix_sum[left] -= 1;
                    sum += 1;
                }
            }
        }
        (heap.len() + queries.len()) as i32
    }
}
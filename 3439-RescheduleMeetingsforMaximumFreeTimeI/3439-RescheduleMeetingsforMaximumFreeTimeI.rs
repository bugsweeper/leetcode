// Last updated: 09.07.2025, 12:08:10
use std::iter::once;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let k = k as usize;
        let free_iter = once(start_time[0])
            .chain(
                end_time
                    .iter()
                    .zip(start_time.iter().skip(1))
                    .map(|(end, start)| *start - *end),
            )
            .chain(once(event_time - *end_time.last().unwrap()));
        let forward_free_iter = free_iter.clone().skip(k + 1);
        let first_sum: i32 = free_iter.clone().take(k + 1).sum();
        free_iter
            .clone()
            .zip(forward_free_iter)
            .fold(
                (first_sum, first_sum),
                |(mut sum, max_sum), (left, right)| {
                    sum += right - left;
                    (sum, max_sum.max(sum))
                },
            )
            .1
    }
}
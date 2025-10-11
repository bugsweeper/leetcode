// Last updated: 11.10.2025, 14:00:58
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut near_queue = VecDeque::with_capacity(3);
        let mut power = power;
        power.sort_unstable();
        let mut iter = power.into_iter();
        let first = iter.next().unwrap();
        near_queue.push_back((first, first as i64));
        let mut far_sum = 0;
        for power in iter {
            let last = near_queue.back_mut().unwrap();
            if last.0 == power {
                last.1 += power as i64;
                continue;
            }
            while let Some(&(prev_power, sum)) = near_queue.front()
                && power - prev_power > 2
            {
                far_sum = far_sum.max(sum);
                near_queue.pop_front();
            }
            near_queue.push_back((power, far_sum + power as i64));
        }
        near_queue.into_iter().map(|(_, sum)| sum).max().unwrap()
    }
}
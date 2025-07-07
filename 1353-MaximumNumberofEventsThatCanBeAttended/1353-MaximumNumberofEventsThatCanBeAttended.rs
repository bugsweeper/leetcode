// Last updated: 07.07.2025, 13:01:04
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events.into_iter().map(|event| (event[0], event[1])).collect::<Vec<_>>();
        events.sort_unstable_by_key(|(start, _)| *start);
        let mut current_day = events[0].0;
        let mut current_pos = 0;
        let mut queue = BinaryHeap::with_capacity(events.len());
        let mut days = 0;
        while current_pos < events.len() {
            if events[current_pos].0 == current_day {
                let next_pos = current_pos + events[current_pos..].partition_point(|(start, _)| *start == current_day);
                queue.extend(events[current_pos..next_pos].iter().map(|(_, end)| Reverse(*end)));
                current_pos = next_pos;
            }
            while let Some(Reverse(end)) = queue.pop() {
                if current_day <= end {
                    days += 1;
                    break;
                }
            }
            current_day += 1;
        }
        while let Some(Reverse(end)) = queue.pop() {
            if current_day <= end {
                days += 1;
                current_day += 1;
            }
        }
        days
    }
}
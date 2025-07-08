// Last updated: 08.07.2025, 16:18:45
impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        if k == 1 {
            return events.into_iter().map(|event| event[2]).max().unwrap();
        }
        let k = k as usize;
        let mut events = events
            .into_iter()
            .map(|event| (event[0], event[1], event[2]))
            .collect::<Vec<_>>();
        events.sort_unstable_by_key(|(_, end, _)| *end);
        let max_events = k + 1;
        let pos_counts = events.len() + 1;
        let mut dp = vec![0; max_events * pos_counts];
        for (i, &(start, _, value)) in events.iter().enumerate() {
            let last_event_index = events[..i].partition_point(|(_, end, _)| *end < start);
            for j in 1..max_events {
                dp[(i + 1) * max_events + j] = dp[i * max_events + j].max(dp[last_event_index * max_events + j - 1] + value);
            }
        }
        *dp.last().unwrap()
    }
}
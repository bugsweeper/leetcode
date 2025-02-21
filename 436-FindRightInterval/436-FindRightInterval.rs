impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut starts: Vec<_> = intervals
            .iter()
            .enumerate()
            .map(|(index, interval)| (index, *unsafe { interval.get_unchecked(0) }))
            .collect();
        starts.sort_unstable_by_key(|(index, start)| *start);
        intervals
            .into_iter()
            .map(|interval| {
                let end = *unsafe { interval.get_unchecked(1) };
                let start_pos = starts.partition_point(|(index, start)| *start < end);
                if start_pos < starts.len() {
                    unsafe { starts.get_unchecked(start_pos) }.0 as i32
                } else {
                    -1
                }
            })
            .collect()
    }
}
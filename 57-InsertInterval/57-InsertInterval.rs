impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let [start, end] = new_interval[..] else {
            return intervals;
        };
        let insert_place =
            intervals.partition_point(|interval| start > *unsafe { interval.get_unchecked(1) });
        let mut intervals = intervals;
        if insert_place == intervals.len() {
            intervals.push(new_interval);
            return intervals;
        }
        let remove_till = insert_place
            + intervals[insert_place..]
                .partition_point(|interval| end >= *unsafe { interval.get_unchecked(0) });
        if insert_place == remove_till {
            intervals.insert(insert_place, new_interval);
            return intervals;
        }
        // First item of replacement becomes united interval, that's why replace end of this interval by end of last item of replacement
        unsafe {
            let new_start = start.min(*intervals.get_unchecked(insert_place).get_unchecked(0));
            let new_end = end.max(*intervals.get_unchecked(remove_till - 1).get_unchecked(1));
            let new_interval = intervals.get_unchecked_mut(insert_place);
            *new_interval.get_unchecked_mut(0) = new_start;
            *new_interval.get_unchecked_mut(1) = new_end;
        };
        if insert_place + 1 < remove_till {
            intervals.drain(insert_place + 1..remove_till);
        }
        intervals
    }
}
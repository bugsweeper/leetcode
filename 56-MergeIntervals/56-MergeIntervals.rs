impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable();
        let [mut current_start, mut current_end] = unsafe { intervals.get_unchecked(0) }[..] else {
            return Vec::new();
        };
        let mut result = Vec::with_capacity(intervals.len());
        for interval in intervals {
            let [start, end] = interval[..] else {
                continue;
            };
            if start > current_end {
                result.push(vec![current_start, current_end]);
                current_start = start;
                current_end = end;
                continue;
            }
            current_end = current_end.max(end);
        }
        result.push(vec![current_start, current_end]);
        result
    }
}

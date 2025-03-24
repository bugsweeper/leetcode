// Last updated: 24.03.2025, 11:25:48
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut last_end = 0;
        let mut poisoned_duration = 0;
        for time_serie in time_series {
            let current_end = time_serie + duration;
            poisoned_duration += current_end - last_end.max(time_serie);
            last_end = current_end;
        }
        poisoned_duration
    }
}
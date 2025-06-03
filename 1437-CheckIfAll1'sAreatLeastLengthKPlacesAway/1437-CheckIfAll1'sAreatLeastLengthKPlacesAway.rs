// Last updated: 03.06.2025, 12:33:07
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut count = 0;
        for (start_time, end_time) in start_time.into_iter().zip(end_time) {
            if start_time <= query_time && query_time <= end_time {
                count += 1;
            }
        }
        count
    }
}
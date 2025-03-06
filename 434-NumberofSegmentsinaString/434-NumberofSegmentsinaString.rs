impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split(' ').filter(|segment| !segment.is_empty()).count() as i32
    }
}
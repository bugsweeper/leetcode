// Last updated: 08.04.2025, 16:38:05
impl Solution {
    pub fn check_record(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut absent = 0;
        let mut late = 0;
        for &day in bytes {
            if day == b'A' {
                absent += 1;
            }
            if day == b'L' {
                late += 1;
            } else {
                late = 0;
            }
            if absent == 2 || late == 3 {
                return false;
            }
        }
        true
    }
}
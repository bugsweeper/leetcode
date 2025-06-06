// Last updated: 06.06.2025, 15:17:47
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut level = 0_usize;
        for log in logs {
            let bytes = log.as_bytes();
            match (bytes[0], bytes[1]) {
                (b'.', b'.') => level = level.saturating_sub(1),
                (b'.', _) => {},
                _ => level += 1,
            }
        }
        level as i32
    }
}
// Last updated: 12.05.2025, 14:59:11
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut iter_name = name
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|slice| (slice[0], slice.len()));
        let mut iter_typed = typed
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|slice| (slice[0], slice.len()));
        loop {
            match (iter_name.next(), iter_typed.next()) {
                (Some((byte_name, count_name)), Some((byte_typed, count_typed))) => {
                    if byte_name != byte_typed || count_name > count_typed {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
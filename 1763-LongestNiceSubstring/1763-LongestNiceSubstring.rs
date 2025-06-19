// Last updated: 19.06.2025, 15:40:19
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        if let Some(index) = s
            .bytes()
            .enumerate()
            .filter_map(|(index, byte)| if byte == b' ' { Some(index) } else { None })
            .nth(k as usize - 1)
        {
            s[..index].into()
        } else {
            s
        }
    }
}
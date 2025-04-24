// Last updated: 24.04.2025, 12:33:42
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|letter| letter.to_ascii_lowercase())
            .collect::<String>()
    }
}
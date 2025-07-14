// Last updated: 14.07.2025, 10:58:32
impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0;
        for num in s
            .split(|token: char| !token.is_ascii_digit())
            .filter_map(|slice| {
                if !slice.is_empty() {
                    slice.parse().ok()
                } else {
                    None
                }
            })
        {
            if num <= prev {
                return false;
            }
            prev = num;
        }
        true
    }
}
// Last updated: 03.06.2025, 16:07:26
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        if n == 0 {
            return "0".into();
        }
        let digits = n.ilog10() as usize + 1;
        let not_separated = n.to_string();
        let mut separated = String::with_capacity(digits + digits / 3);
        let mut shift = digits % 3;
        let mut remaining = &not_separated[..];
        if shift == 0 {
            shift = 3;
        }
        separated.push_str(&remaining[..shift]);
        remaining = &remaining[shift..];
        while !remaining.is_empty() {
            separated.push('.');
            separated.push_str(&remaining[..3]);
            remaining = &remaining[3..];
        }
        separated
    }
}
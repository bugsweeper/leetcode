// Last updated: 22.06.2025, 14:17:37
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let mut remaining = s.as_str();
        let mut divided = Vec::with_capacity(s.len().div_ceil(k));
        while remaining.len() > k {
            let (first, rest) = remaining.split_at(k);
            divided.push(first.into());
            remaining = rest;
        }
        divided.push(if remaining.len() == k {
            remaining.into()
        } else {
            let mut last = String::with_capacity(k);
            last.push_str(remaining);
            last.extend(std::iter::repeat_n(fill, k - remaining.len()));
            last
        });
        divided
    }
}
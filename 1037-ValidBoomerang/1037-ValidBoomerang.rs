// Last updated: 19.05.2025, 11:51:24
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        for byte in s.as_bytes() {
            if let Some(last) = stack.last() {
                if last == byte {
                    stack.pop();
                } else {
                    stack.push(*byte);
                }
            } else {
                stack.push(*byte);
            }
        }
        String::from_utf8(stack).unwrap()
    }
}
// Last updated: 08.04.2025, 16:14:43
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s;
        let bytes = unsafe{ s.as_bytes_mut() };
        let k = k as usize;
        for start in (0..bytes.len()).step_by(2 * k) {
            let end = k.min(bytes.len() - start) - 1;
            for i in 0..=end / 2 {
                bytes.swap(start + i, start + end - i);
            }
        }
        s
    }
}
// Last updated: 03.06.2025, 14:18:40
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![b' '; s.len()];
        for (index, &byte) in indices.into_iter().zip(s.as_bytes()) {
            result[index as usize] = byte;
        }
        String::from_utf8(result).unwrap()
    }
}
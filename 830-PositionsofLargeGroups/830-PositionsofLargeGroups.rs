// Last updated: 02.05.2025, 16:17:50
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(s.len() / 3);
        let mut prev_char = b'a';
        let mut prev_index = 0;
        for (cur_index, &cur_char) in s.as_bytes().iter().enumerate() {
            if cur_char != prev_char {
                if cur_index - prev_index >= 3 {
                    result.push(vec![prev_index as i32, cur_index as i32 - 1]);
                }
                (prev_index, prev_char) = (cur_index, cur_char)
            }
        }
        if s.len() - prev_index >= 3 {
            result.push(vec![prev_index as i32, s.len() as i32 - 1]);
        }
        result
    }
}
// Last updated: 02.05.2025, 15:15:30
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let b = c as u8;
        let mut iter = s
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(index, &byte)| if byte == b { Some(index) } else { None });
        let mut result = Vec::with_capacity(s.len());
        let mut prev_pos = iter.next().unwrap();
        result.extend((0..=prev_pos as i32).rev());
        while let Some(pos) = iter.next() {
            let count = pos - prev_pos;
            result.extend(1..(count as i32 + 2) / 2);
            result.extend((0..(count as i32 + 1) / 2).rev());
            prev_pos = pos;
        }
        result.extend(1..=(s.len() - prev_pos - 1) as i32);
        result
    }
}
// Last updated: 07.06.2025, 13:49:33
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut bytes = s.into_bytes();
        let mut last_least_bytes = BinaryHeap::with_capacity(bytes.len());
        for i in 0..bytes.len() {
            let cur_byte = bytes[i];
            if cur_byte == b'*' {
                let (_, prev_least_byte_index) = last_least_bytes.pop().unwrap();
                bytes[prev_least_byte_index] = b'*';
            } else {
                last_least_bytes.push((Reverse(cur_byte), i));
            }
        }
        bytes.into_iter().filter_map(|byte| if byte == b'*' { None } else { Some(byte as char) }).collect::<String>()
    }
}
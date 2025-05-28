// Last updated: 28.05.2025, 15:57:34
use std::collections::VecDeque;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut buffer = VecDeque::with_capacity(a.len().max(b.len()) + 1);
        let mut next_position_bit = 0;
        let mut current_bit;
        let min_size = a.len().min(b.len());
        let max_size = a.len().max(b.len());
        let mut a_iter = a.as_bytes().iter().rev();
        let mut b_iter = b.as_bytes().iter().rev();
        for _ in 0..min_size {
            let (&a, &b) = (a_iter.next().unwrap(), b_iter.next().unwrap());
            (next_position_bit, current_bit) = match (a, b, next_position_bit) {
                (b'0', b'0', 0) => (0, b'0'),
                (b'1', b'1', 1) => (1, b'1'),
                (b'0', b'0', 1) | (b'0', b'1', 0) | (b'1', b'0', 0) => (0, b'1'),
                _ => (1, b'0'),
            };
            buffer.push_front(current_bit);
        }
        let mut single_iter = if a.len() > b.len() { a_iter } else { b_iter };
        for _ in min_size..max_size {
            if next_position_bit == 0 {
                for &a in single_iter {
                    buffer.push_front(a);
                }
                break;
            } else if *single_iter.next().unwrap() == b'0' {
                buffer.push_front(b'1');
                next_position_bit = 0;
            } else {
                buffer.push_front(b'0');
            }
        }
        if next_position_bit == 1 {
            buffer.push_front(b'1');
        }
        String::from_utf8(buffer.into()).unwrap()
    }
}
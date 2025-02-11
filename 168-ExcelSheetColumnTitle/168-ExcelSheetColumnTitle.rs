use std::collections::VecDeque;

const ABC_LEN: i32 = (b'Z' - b'A' + 1) as i32;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number - 1;
        let mut result = VecDeque::with_capacity(7);
        while column_number >= 0 {
            let letter = (column_number % ABC_LEN) as u8 + b'A';
            result.push_front(letter);
            column_number = column_number / ABC_LEN - 1;
        }
        unsafe{String::from_utf8_unchecked(result.into())}
    }
}
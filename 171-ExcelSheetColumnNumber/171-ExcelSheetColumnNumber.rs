use std::collections::VecDeque;

const ABC_LEN: i32 = (b'Z' - b'A' + 1) as i32;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut number = 0;
        for &letter in column_title.as_bytes() {
            number *= ABC_LEN;
            number += (letter - b'A' + 1) as i32;
        }
        number
    }
}
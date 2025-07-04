// Last updated: 04.07.2025, 10:22:05
const ABC_LEN: u8 = b'z' - b'a' + 1;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut shifts = 0;
        let mut k = k - 1;
        for operation in operations {
            if operation == 1 && k & 1 == 1 {
                shifts += 1;
            }
            k >>= 1;
            if k == 0 {
                break;
            }
        }
        (shifts % ABC_LEN + b'a') as char
    }
}
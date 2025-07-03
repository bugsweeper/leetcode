// Last updated: 03.07.2025, 11:23:47
impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut character = (k - 1).count_ones() as u8 + b'a';
        // for k <= 500 it's imposible to get character more than 'j'
        // but the most correct solution should account 'z' -> 'a' rotation
        if character > b'z' {
            character -= b'z' - b'a' + 1;
        }
        character as char
    }
}
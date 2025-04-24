// Last updated: 24.04.2025, 12:39:44
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut index = 0;
        let mut result = false;
        while index < bits.len() {
            if bits[index] == 0 {
                result = true;
                index += 1;
            } else {
                result = false;
                index += 2;
            };
        }
        result
    }
}
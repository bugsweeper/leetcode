// Last updated: 03.06.2025, 13:34:46
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut sum = 0;
        for num in (start..start + (n << 1)).step_by(2) {
            sum ^= num;
        }
        sum
    }
}
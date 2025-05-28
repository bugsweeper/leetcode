// Last updated: 28.05.2025, 16:40:36
impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut position = 10_i32.pow(num.ilog10() + 1);
        while position > 0 {
            let digit = num / position % 10;
            if digit == 6 {
                return num + 3 * position;
            }
            position /= 10;
        }
        num
    }
}
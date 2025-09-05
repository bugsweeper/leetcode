// Last updated: 05.09.2025, 22:43:40
impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let mut sum = num1 as i64;
        for i in 1..37 {
            sum -= num2 as i64;
            if sum < i {
                return -1;
            }
            if sum.count_ones() as i64 <= i {
                return i as i32;
            }
        }
        -1
    }
}
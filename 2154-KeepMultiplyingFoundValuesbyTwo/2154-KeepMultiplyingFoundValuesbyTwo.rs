// Last updated: 01.08.2025, 11:30:43
impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut count = 0;
        let (mut num1, mut num2) = (num1, num2);
        while num1 != 0 && num2 != 0 {
            if num1 > num2 {
                num1 -= num2;
            } else {
                num2 -= num1;
            }
            count += 1;
        }
        count
    }
}
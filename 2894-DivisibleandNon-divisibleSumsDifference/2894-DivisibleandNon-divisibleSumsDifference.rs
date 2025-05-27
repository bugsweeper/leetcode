// Last updated: 27.05.2025, 08:09:16
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let (num1, num2) = (1..=n).fold((0, 0), |(num1, num2), num| {
            if num % m == 0 {
                (num1, num2 + num)
            } else {
                (num1 + num, num2)
            }
        });
        num1 - num2
    }
}
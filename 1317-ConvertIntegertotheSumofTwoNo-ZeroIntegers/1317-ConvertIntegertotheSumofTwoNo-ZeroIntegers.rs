// Last updated: 08.09.2025, 09:05:57
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut n = n - 1;
        let mut position = 1;
        let mut pair = 1;
        while n > position {
            n -= position;
            pair += position;
            if n < position {
                break;
            }
            let digit = n / position % 10;
            if digit == 0 {
                n -= position;
                pair += position;
            }
            position *= 10;
        }
        vec![n, pair]
    }
}
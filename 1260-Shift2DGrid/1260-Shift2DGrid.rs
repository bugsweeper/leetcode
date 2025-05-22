// Last updated: 22.05.2025, 14:50:35
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut arr = Vec::with_capacity(n as usize);
        arr.extend(-n / 2..0);
        if n & 1 == 1 {
            arr.push(0);
        }
        arr.extend(1..=n / 2);
        arr
    }
}
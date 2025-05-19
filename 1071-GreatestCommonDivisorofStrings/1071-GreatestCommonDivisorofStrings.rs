// Last updated: 19.05.2025, 15:53:27
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let (mut t0, mut t1, mut t2) = (0, 1, 1);
                for _ in 2..n {
                    (t0, t1, t2) = (t1, t2, t0 + t1 + t2);
                }
                t2
            }
        }
    }
}
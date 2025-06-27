// Last updated: 27.06.2025, 11:21:53
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            _ => {
                // Fibonacci
                let mut a = 1;
                let mut b = 2;
                for _ in 0..n - 2 {
                    (a, b) = (b, a + b);
                }
                b
            }
        }
    }
}
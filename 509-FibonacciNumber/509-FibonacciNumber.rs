// Last updated: 01.04.2025, 13:41:40
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let (mut f1, mut f2) = (0, 1);
        for _ in 0..n-1 {
            (f1, f2) = (f2, f1 + f2);
        }
        f2
    }
}
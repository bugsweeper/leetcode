// Last updated: 05.05.2025, 12:23:01
const MODULO: u32 = 1_000_000_007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let (mut x0prev, mut x1prev, mut x2prev, mut x0, mut x1, mut x2) = (1, 0, 0, 1, 1, 1);
        for _ in 2..n {
            (x0prev, x1prev, x2prev, x0, x1, x2) = (x0, x1, x2, (x0prev + x0 + x1prev + x2prev) % MODULO, (x0 + x2) % MODULO, (x0 + x1) % MODULO);
        }
        ((x0prev + x0 + x1prev + x2prev) % MODULO) as i32
    }
}
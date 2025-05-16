// Last updated: 16.05.2025, 23:39:39
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        if n == 1 {
            return false;
        }
        let n = n as usize;
        let mut wins = vec![false; n + 1];
        wins[2] = true;
        for i in 3..=n {
            for j in 1..i {
                if !wins[i - j] && i % j == 0 {
                    wins[i] = true;
                    break;
                }
            }
        }
        wins[n]
    }
}
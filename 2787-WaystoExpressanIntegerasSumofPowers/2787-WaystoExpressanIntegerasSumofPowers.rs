// Last updated: 12.08.2025, 12:59:36
const MODULO: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let (n, x) = (n as usize, x as u32);
        let mut dp = vec![vec![0; n + 1]; n + 1];
        dp[0][0] = 1;
        dp[1][1] = 1;
        for i in 2..=n {
            let mut prev = 0;
            for j in 1..=i {
                let addition = j.pow(x);
                if addition > i {
                    dp[i][j] = prev;
                    continue;
                }
                let subsum = i - addition;
                let cur = (prev/* Do not include j^x */ + dp[subsum][subsum.min(j - 1)] /* Include j^x as max addition */) % MODULO;
                dp[i][j] = cur;
                prev = cur;
            }
        }
        dp[n][n]
    }
}
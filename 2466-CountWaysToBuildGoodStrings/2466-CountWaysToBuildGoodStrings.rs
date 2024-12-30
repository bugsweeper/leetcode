const MODULO: i32 = 1_000_000_007;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;
        let mut ways = vec![0; high + 1];
        ways[0] = 1;
        let mut answer = 0;
        for length in zero.min(one)..=high {
            if length >= zero {
                ways[length] += ways[length - zero] % MODULO;
            }
            if length >= one {
                ways[length] += ways[length - one] % MODULO;
            }
            if length >= low {
                answer += ways[length] % MODULO;
            }
            answer %= MODULO;
        }
        answer
    }
}
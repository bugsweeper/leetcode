// Last updated: 23.04.2025, 01:32:14
const MODULO: i32 = 1_000_000_007;
const MODULO64: i64 = MODULO as i64;

const MULTIPLICATIVE_INVERSE_FACTORIAL: [i64; 14] = [
    1, // 1! = 1
    500000004, // 1/2! = 1/2
    166666668, // 1/3! = 1/6
    41666667, // 1/4! = 1/24
    808333339, // 1/5! = 1/120
    301388891, // 1/6! = 1/720
    900198419, // 1/7! = 1/5040
    487524805, // 1/8! = 1/40320
    831947206, // 1/9! = 1/362880
    283194722, // 1/10! = 1/3628800
    571199524, // 1/11! = 1/39916800
    380933296, // 1/12! = 1/479001600
    490841026, // 1/13! = 1/6227020800
    320774361, // 1/14! = 1/87178291200
];

/// c(m,n) = n!/m!(n-m)!
fn c(m: usize, n: i32) -> i64 {
    let (m, n) = (m as i64, n as i64);
    if m == 0 || m == n {
        return 1;
    }
    if m > n {
        return 0;
    }
    let mut result = 1;
    for i in 0..m.min(n - m) {
        result = (result * (n - i)) % MODULO64;
    }
    (result * MULTIPLICATIVE_INVERSE_FACTORIAL[m.min(n - m) as usize - 1]) % MODULO64
}

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        if max_value == 1 {
            return 1;
        }
        let mut final_multipliers =
            Vec::with_capacity((i32::BITS - max_value.leading_zeros()) as usize);
        final_multipliers.push(1);
        final_multipliers.push(max_value - 1);
        let mut combinations = (0..max_value / 2)
            .map(|multiplier| max_value / (multiplier + 1) - 1)
            .collect::<Vec<_>>();
        while final_multipliers.len() < final_multipliers.capacity() {
            for i in 0..combinations.len() / 2 {
                combinations[i] = combinations
                    .iter()
                    .skip(2 * i + 1)
                    .step_by(i + 1)
                    .copied()
                    .reduce(|acc, value| (acc + value) % MODULO)
                    .unwrap_or_default();
            }
            combinations.truncate(combinations.len() / 2);
            final_multipliers.push(combinations[0]);
        }
        final_multipliers
            .into_iter()
            .enumerate()
            .map(|(index, multiplier)| ((multiplier as i64 * c(index, n)) % MODULO64) as i32)
            .reduce(|acc, value| (acc + value) % MODULO)
            .unwrap_or_default()
    }
}
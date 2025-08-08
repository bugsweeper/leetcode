// Last updated: 08.08.2025, 22:43:22
use std::collections::HashMap;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n > 4150 {
            // for n = 4175 answer is 0.9999813231014011 which is in double accepted error from max value
            return 0.99999;
        }
        // Moving from calculating mL to calculating portions, rounding to greeter
        let n = ((n + 24) / 25) as usize;
        let start_value = if n & 1 == 0 {
            [4, 3, 2, 1]
        } else {
            [2, 1, 4, 3]
        };
        let mut dp = HashMap::with_capacity((n * n) >> 4);
        dp.extend([
            ((0, 0), 0.5),
            ((1, 1), 0.625),
            ((1, 3), 0.875),
            ((2, 2), 0.625),
            ((2, 4), 0.84375),
            ((3, 1), 0.375),
            ((3, 3), 0.65625),
            ((4, 2), 0.34375),
            ((4, 4), 0.71875),
        ]);
        if let Some(&probability) = dp.get(&(n, n)) {
            return probability;
        }
        for i in 1..=n {
            let mut start_value = start_value[i & 3];
            if i << 2 > n * 3 {
                start_value = n - ((n - i) * 3)
            }
            for j in (start_value..=n).step_by(4) {
                let mut sum = 0.;
                for (shift_i, shift_j) in [(4, 0), (3, 1), (2, 2), (1, 3)] {
                    let coord = (i.saturating_sub(shift_i), j.saturating_sub(shift_j));
                    sum += if let Some(&probability) = dp.get(&coord) {
                        probability
                    } else if coord.0 == 0 {
                        1.0
                    } else if coord.1 == 0 {
                        0.0
                    } else {
                        unimplemented!("{n}) {coord:?} = ({i}, {j}) - ({shift_i}, {shift_j})");
                    }
                }
                dp.insert((i, j), sum / 4.0);
            }
        }
        dp[&(n, n)]
    }
}
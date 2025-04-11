// Last updated: 11.04.2025, 09:28:05
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut count = 0;
        if low < 100 {
            for number in (11..=99.min(high)).step_by(11) {
                if number >= low {
                    count += 1;
                }
            }
        }
        if high > 1000 {
            'hundreds: for hundreds in 10.max(low / 100)..=99 {
                let digit_sum = hundreds % 10 + hundreds / 10;
                let first = if digit_sum > 9 {
                    10 * digit_sum - 81
                } else {
                    digit_sum
                } + hundreds * 100;
                if first > high {
                    break;
                }
                let last = (hundreds + 1) * 100;
                for number in (first..last).step_by(9) {
                    if number > high {
                        break 'hundreds;
                    }
                    if number >= low {
                        count += 1;
                    }
                    if number % 10 == 0 {
                        break;
                    }
                }
            }
        }
        count
    }
}
// Last updated: 15.06.2025, 11:58:18
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let power_of_10 = num.ilog10();
        let mut powered_10 = 10_i32.pow(power_of_10);
        let first = num / powered_10;
        let mut first_non_9 = first;
        if first == 9 {
            let mut remaining = num;
            let mut powered_10 = powered_10;
            while powered_10 > 0 {
                let digit = remaining / powered_10;
                if digit != 9 {
                    first_non_9 = digit;
                    break;
                }
                remaining %= powered_10;
                powered_10 /= 10;
            }
        }
        let (low_replaced, low_replacement) = if first == 1 {
            let mut remaining = num;
            let mut powered_10 = powered_10;
            let mut second_non_zero = 0;
            while powered_10 > 0 {
                let digit = remaining / powered_10;
                if digit != first && digit != 0 {
                    second_non_zero = digit;
                    break;
                }
                remaining %= powered_10;
                powered_10 /= 10;
            }
            (second_non_zero, 0)
        } else {
            (first, 1)
        };
        let mut high = 0;
        let mut low = 0;
        let mut remaining = num;
        while powered_10 > 0 {
            let digit = remaining / powered_10;
            remaining %= powered_10;
            powered_10 /= 10;
            high = 10 * high + if digit == first_non_9 { 9 } else { digit };
            low = 10 * low
                + if digit == low_replaced {
                    low_replacement
                } else {
                    digit
                };
        }
        high - low
    }
}
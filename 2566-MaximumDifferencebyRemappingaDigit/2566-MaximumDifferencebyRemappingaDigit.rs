// Last updated: 14.06.2025, 14:03:35
impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
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
        let mut high = 0;
        let mut low = 0;
        let mut remaining = num;
        while powered_10 > 0 {
            let digit = remaining / powered_10;
            remaining %= powered_10;
            powered_10 /= 10;
            high = 10 * high + if digit == first_non_9 {
                9
            } else {
                digit
            };
            low = 10 * low + if digit == first {
                0
            } else {
                digit
            }
        }
        high - low
    }
}
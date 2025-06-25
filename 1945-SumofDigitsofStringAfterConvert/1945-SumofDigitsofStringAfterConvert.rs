// Last updated: 25.06.2025, 08:52:14
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let shift = b'a' - 1;
        let mut sum = 0;
        for mut byte in s.bytes() {
            byte -= shift;
            sum += (byte / 10 + byte % 10) as i32;
        }
        for _ in 1..k {
            if sum < 10 {
                break;
            }
            let mut digit_sum = 0;
            while sum > 0 {
                digit_sum += sum % 10;
                sum /= 10;
            }
            sum = digit_sum;
        }
        sum
    }
}
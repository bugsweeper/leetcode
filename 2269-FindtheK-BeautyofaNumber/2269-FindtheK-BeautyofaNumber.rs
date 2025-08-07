// Last updated: 07.08.2025, 12:28:13
impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut remaining = num;
        let divisor10 = 10_i32.pow(k as u32);
        let mut answer = 0;
        while remaining >= divisor10 {
            let possible_divisor = remaining % divisor10;
            if possible_divisor != 0 && num % possible_divisor == 0 {
                answer += 1;
            }
            remaining /= 10;
        }
        if num % remaining == 0 {
            answer += 1;
        }
        answer
    }
}
// Last updated: 31.03.2025, 15:33:59
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".into()
        }
        let add_minus = num < 0;
        let mut num = num.abs();
        let mut result = Vec::with_capacity(9);
        while num > 0 {
            result.push(((num % 7) as u8 + b'0') as char);
            num /= 7;
        }
        if add_minus {
            result.push('-');
        }
        result.into_iter().rev().collect::<String>()
    }
}
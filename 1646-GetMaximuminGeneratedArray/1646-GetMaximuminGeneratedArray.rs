// Last updated: 16.06.2025, 13:15:48
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut digits = number
            .bytes()
            .filter(|byte| byte.is_ascii_digit())
            .count();
        let mut iter = number
            .bytes()
            .filter(|byte| byte.is_ascii_digit());
        let mut group_size = 0;
        let mut result = String::with_capacity(number.len() * 3 / 2);
        while let Some(byte) = iter.next() {
            group_size += 1;
            digits -= 1;
            result.push(byte as char);
            if group_size == 3 && digits != 0 || group_size == 2 && digits == 2 {
                result.push('-');
                group_size = 0;
            }
        }
        result
    }
}
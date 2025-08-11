// Last updated: 11.08.2025, 13:13:41
impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count = [0; 10];
        for byte in num.bytes() {
            count[(byte - b'0') as usize] += 1;
        }
        num.bytes().enumerate().all(|(index, byte)| count[index] == byte - b'0')
    }
}
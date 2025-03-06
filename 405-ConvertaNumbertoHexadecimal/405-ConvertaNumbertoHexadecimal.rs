impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut result = String::new();
        for i in (0..i32::BITS / u8::BITS * 2).rev() {
            let bits = ((num >> ((i * u8::BITS) >> 1)) & 0xf) as u8;
            if bits > 0 || !result.is_empty() {
                result.push(if bits > 9 {
                    (bits - 10) + b'a'
                } else {
                    bits + b'0'
                } as char);
            }
        }
        if result.is_empty() {
            result.push('0');
        }
        result
    }
}
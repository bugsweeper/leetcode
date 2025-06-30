// Last updated: 30.06.2025, 11:15:03
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut prev_byte = b' ';
        let mut bytes_met = 0;
        let mut fancy = String::with_capacity(s.len());
        for byte in s.bytes() {
            if prev_byte == byte {
                if bytes_met < 2 {
                    bytes_met += 1;
                } else {
                    continue;
                }
            } else {
                (prev_byte, bytes_met) = (byte, 1);
            }
            fancy.push(byte as char);
        }
        fancy
    }
}
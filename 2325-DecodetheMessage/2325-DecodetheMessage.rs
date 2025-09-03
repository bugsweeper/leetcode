// Last updated: 03.09.2025, 11:11:05
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut dictionary = vec![' '; (b'z' + 1) as usize];
        let mut cur_value = b'a';
        for key in key.bytes() {
            if key == b' ' {
                continue;
            }
            let value = &mut dictionary[key as usize];
            // check if already filled
            if *value == ' ' {
                *value = cur_value as char;
                if cur_value == b'z' {
                    break;
                }
                cur_value += 1;
            }
        }
        message.bytes().map(|key| dictionary[key as usize]).collect::<String>()
    }
}
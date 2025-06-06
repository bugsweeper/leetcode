// Last updated: 06.06.2025, 10:46:31
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut count = [0; ABC_LEN];
        for byte in s.as_bytes() {
            count[(byte - b'a') as usize] += 1;
        }
        let mut stack = Vec::with_capacity(s.len());
        let bytes = s.into_bytes();
        let mut current_byte_index = 0;
        let mut result = String::with_capacity(bytes.len());
        for lowest_letter_index in 0..ABC_LEN {
            let lowest_letter = lowest_letter_index as u8 + b'a';
            while let Some(&top) = stack.last() {
                if top > lowest_letter {
                    break;
                }
                result.push(top as char);
                stack.pop();
            }
            if count[lowest_letter_index] == 0 {
                continue;
            }
            while current_byte_index < bytes.len() {
                let current_byte = bytes[current_byte_index];
                current_byte_index += 1;
                let count = &mut count[(current_byte - b'a') as usize];
                *count -= 1;
                if current_byte == lowest_letter {
                    result.push(current_byte as char);
                    if *count == 0 {
                        break;
                    }
                } else {
                    stack.push(current_byte);
                }
            }
        }
        while let Some(letter) = stack.pop() {
            result.push(letter as char);
        }
        result
    }
}
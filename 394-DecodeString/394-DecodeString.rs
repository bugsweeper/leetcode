// Last updated: 16.05.2025, 15:44:09
fn decode_string(slice: &str) -> String {
    let mut result = String::with_capacity(slice.len());
    let mut starting_index = 0;
    while let Some(mut first_digit) = slice[starting_index..].find(|c: char | c.is_ascii_digit()) {
        first_digit += starting_index;
        if first_digit > starting_index {
            result.push_str(&slice[starting_index..first_digit]);
        }
        let open_bracket = slice[first_digit..].find('[').unwrap() + first_digit;
        let k: usize = slice[first_digit..open_bracket].parse().unwrap();
        let mut brackets = 1;
        let mut close_bracket = open_bracket + 2;
        for (index, &byte) in slice.as_bytes().iter().enumerate().skip(open_bracket + 1) {
            match byte {
                b'[' => brackets += 1,
                b']' => {
                    brackets -= 1;
                    if brackets == 0 {
                        close_bracket = index;
                        break;
                    }
                }
                _ => {}
            }
        }
        let repeated = decode_string(&slice[open_bracket + 1..close_bracket]);
        result.extend(std::iter::repeat_n(repeated, k));
        starting_index = close_bracket + 1;
    }
    if starting_index < slice.len() {
        result.push_str(&slice[starting_index..]);
    }
    result
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        decode_string(&s)
    }
}
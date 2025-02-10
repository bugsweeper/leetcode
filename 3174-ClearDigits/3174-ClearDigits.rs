impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut s = s;
        let s_bytes = unsafe { s.as_mut_vec() };
        loop {
            if s_bytes.is_empty() {
                break;
            }
            let mut prev_byte = *s_bytes.first().unwrap();
            let Some(mut remove_point) = s_bytes.iter().skip(1).position(|&byte| {
                let result = !prev_byte.is_ascii_digit() && byte.is_ascii_digit();
                prev_byte = byte;
                result
            }) else {
                break;
            };
            // Skipped first byte, compensate it
            remove_point += 1;
            let left_iter = s_bytes[..remove_point].iter().rev().enumerate().skip(1);
            let right_iter = s_bytes[remove_point..].iter().skip(1);
            let mut count = 0;
            for ((index, &letter), &digit) in left_iter.zip(right_iter) {
                if !letter.is_ascii_digit() && digit.is_ascii_digit() {
                    count = index;
                } else {
                    break;
                }
            }
            // Counted additional letters, compensate it
            count += 1;
            s_bytes.drain(remove_point - count..remove_point + count);
        }
        s
    }
}
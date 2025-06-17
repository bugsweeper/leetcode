// Last updated: 17.06.2025, 14:33:28
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let bytes = time.as_bytes();
        let mut result = String::with_capacity(5);
        match bytes[0] {
            b'?' => match bytes[1] {
                b'?' => result.push_str("23"),
                digit => {
                    if digit < b'4' {
                        result.push('2');
                    } else {
                        result.push('1');
                    }
                    result.push(digit as char);
                }
            }
            digit => {
                result.push(digit as char);
                match bytes[1] {
                    b'?' => if digit == b'2' {
                        result.push('3');
                    } else {
                        result.push('9');
                    }
                    digit => result.push(digit as char),
                }
            }

        }
        result.push(':');
        match bytes[3] {
            b'?' => result.push('5'),
            digit => result.push(digit as char),
        }
        match bytes[4] {
            b'?' => result.push('9'),
            digit => result.push(digit as char),
        }
        result
    }
}
// Last updated: 11.08.2025, 13:43:09
impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }
        let mut prev_byte = b' ';
        let mut has_lowercase = false;
        let mut has_uppercase = false;
        let mut has_digit = false;
        let mut has_special = false;
        for byte in password.bytes() {
            if byte == prev_byte {
                return false;
            }
            if byte.is_ascii_digit() {
                has_digit = true;
            } else if byte.is_ascii_lowercase() {
                has_lowercase = true;
            } else if byte.is_ascii_uppercase() {
                has_uppercase = true;
            } else {
                has_special = true;
            }
            prev_byte = byte;
        }
        has_digit && has_lowercase && has_uppercase && has_special
    }
}
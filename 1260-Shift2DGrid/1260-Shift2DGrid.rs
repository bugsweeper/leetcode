// Last updated: 22.05.2025, 15:31:16
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut decrypted = Vec::with_capacity(s.len());
        let mut iter = s.as_bytes().iter().rev();
        while let Some(&byte) = iter.next() {
            if byte == b'#' {
                decrypted.push(*iter.next().unwrap() - b'0' + 10 * (*iter.next().unwrap() - b'1') + b'j');
            } else {
                decrypted.push(byte - b'1' + b'a');
            }
        }
        decrypted.into_iter().map(|byte| byte as char).rev().collect::<String>()
    }
}
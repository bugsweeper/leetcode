impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s_bytes = s.as_bytes();
        let mut s_iter = s_bytes.iter();
        let mut answer = Vec::with_capacity(s.len());
        while let Some(end) = s_iter.rposition(|&symbol| symbol != b' ') {
            if !answer.is_empty() {
                answer.push(b' ');
            }
            answer.extend_from_slice(if let Some(start) = s_iter.rposition(|&symbol| symbol == b' ') {
                &s_bytes[start + 1..end + 1]
            } else {
                &s_bytes[0..end + 1]
            });
        }
        unsafe {
            String::from_utf8_unchecked(answer)
        }
    }
}
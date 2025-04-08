// Last updated: 08.04.2025, 16:47:24
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s;
        let bytes = unsafe{ s.as_bytes_mut() };
        let mut start = 0;
        for index in 0..bytes.len() {
            if bytes[index] == b' ' {
                let end = index - 1;
                for i in 0..=(end - start) / 2 {
                    bytes.swap(start + i, end - i)
                }
                start = index + 1;
            }
        }
        let end = bytes.len() - 1;
        for i in 0..=(end - start) / 2 {
            bytes.swap(start + i, end - i)
        }
        s
    }
}
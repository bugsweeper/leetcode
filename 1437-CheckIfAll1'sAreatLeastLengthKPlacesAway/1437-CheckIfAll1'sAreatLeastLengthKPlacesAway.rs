// Last updated: 04.06.2025, 13:16:27
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut c1 = b' ';
        for slice in s.as_bytes().windows(2) {
            let (mut c2, c3) = (slice[0], slice[1]);
            if c2 != b'?' {
                result.push(c2 as char);
                c1 = c2;
                continue;
            }
            c2 = match (c1, c3) {
                (b'a', b'b') | (b'b', b'a') => b'c',
                (b'a', _) | (_, b'a') => b'b',
                _ => b'a',
            };
            result.push(c2 as char);
            c1 = c2;
        }
        let last = *s.as_bytes().last().unwrap();
        result.push(if last == b'?' {
            if c1 == b'a' {
                'b'
            } else {
                'a'
            }
        } else {
            last as char
        });
        result
    }
}
// Last updated: 06.05.2025, 12:30:17
fn to_iter(bytes: &[u8]) -> impl Iterator<Item = &u8> {
    let mut backspaces = 0;
    bytes.iter().rev().filter_map(move |byte| {
        if *byte == b'#' {
            backspaces += 1;
            None
        } else if backspaces == 0 {
            Some(byte)
        } else {
            backspaces -= 1;
            None
        }
    })
}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut s_iter, mut t_iter) = (to_iter(s.as_bytes()), to_iter(t.as_bytes()));
        loop {
            match (s_iter.next(), t_iter.next()) {
                (None, None) => return true,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(&s_byte), Some(&t_byte)) => {
                    if s_byte != t_byte {
                        return false;
                    }
                }
            }
        }
    }
}
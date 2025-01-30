#[inline]
fn open_pair(closed_bracket: u8) -> u8 {
    match closed_bracket {
        b')' => b'(',
        b'}' => b'{',
        b']' => b'[',
        any_other => any_other,
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let s = s.as_bytes();
        for &bracket in s {
            match bracket {
                b'(' | b'{' | b'[' => stack.push(bracket),
                closed_bracket => {
                    if let Some(open_bracket) = stack.pop() {
                        if open_bracket != open_pair(closed_bracket) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        return stack.is_empty()
    }
}
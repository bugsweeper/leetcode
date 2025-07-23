// Last updated: 23.07.2025, 10:17:05
#[inline]
fn remove_substring(source: &mut Vec<u8>, a: u8, b: u8) -> i32 {
    let mut stack = Vec::with_capacity(source.len());
    for &byte in source.iter() {
        if byte == b {
            if let Some(&last) = stack.last() {
                if last == a {
                    stack.pop();
                    continue;
                }
            }
        }
        stack.push(byte);
    }
    std::mem::swap(&mut stack, source);
    ((stack.len() - source.len()) >> 1) as i32
}

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        // greedy moment: select which 'ab' or 'ba' removing first depending on reward
        let (a, b, x, y) = if x > y { (b'a', b'b', x, y) } else { (b'b', b'a', y, x) };
        let mut source = s.into_bytes();
        remove_substring(&mut source, a, b) * x + remove_substring(&mut source, b, a) * y
    }
}
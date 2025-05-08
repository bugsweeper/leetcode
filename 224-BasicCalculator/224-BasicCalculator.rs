// Last updated: 08.05.2025, 11:03:46
fn calculate<'byte>(iter: &mut impl Iterator<Item = &'byte u8>) -> i32 {
    let mut sum = 0;
    let mut number = 0;
    let mut sign = b'+';
    while let Some(&c) = iter.next() {
        if c.is_ascii_digit() {
            number = number * 10 + (c - b'0') as i32;
            continue;
        }
        if c == b' ' {
            continue;
        }
        sum += if sign == b'-' { -number } else { number };
        number = 0;
        match c {
            b'(' => number = calculate(iter),
            b')' => return sum,
            _ => sign = c,
        }
    }
    sum + if sign == b'-' { -number } else { number }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        calculate(&mut s.as_bytes().iter())
    }
}
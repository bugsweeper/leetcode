// Last updated: 19.05.2025, 13:18:56
fn consist_of(mut goal: &str, cd: &str) -> bool {
    loop {
        let (head, remaining) = goal.split_at(cd.len());
        if head != cd {
            return false;
        }
        if remaining.is_empty() {
            return true;
        }
        goal = remaining;
    }
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (long, short) = if str1.len() > str2.len() {
            (str1, str2)
        } else {
            (str2, str1)
        };
        if long
            .as_bytes()
            .iter()
            .zip(short.as_bytes())
            .any(|(byte1, byte2)| byte1 != byte2)
        {
            return String::new();
        }
        if long.len() == short.len() {
            return short;
        }
        let (mut len, mut gcd_len) = (long.len(), short.len());
        while len % gcd_len != 0 {
            (len, gcd_len) = (gcd_len, len % gcd_len);
        }
        for cd_len in (1..=gcd_len).rev() {
            if gcd_len % cd_len != 0 {
                continue;
            }
            let cd = &short[..cd_len];
            if consist_of(&long, cd) {
                return cd.into();
            }
        }
        String::new()
    }
}
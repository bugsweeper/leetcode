// Last updated: 08.04.2025, 14:44:41
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut iter = word
            .as_bytes()
            .into_iter()
            .map(|byte| byte.is_ascii_uppercase());
        match iter.next().unwrap() {
            false => iter.all(|uppercase| !uppercase),
            true => match iter.next() {
                None => true,
                Some(second) => iter.all(|uppercase| uppercase == second),
            },
        }
    }
}
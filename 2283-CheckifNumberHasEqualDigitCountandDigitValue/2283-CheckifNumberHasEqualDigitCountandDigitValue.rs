// Last updated: 11.08.2025, 14:02:31
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut exists = vec![false; (b'z' + 1) as usize];
        for byte in s.bytes() {
            exists[byte as usize] = true;
        }
        if let Some(index) = exists[b'A' as usize..]
            .iter()
            .zip(&exists[b'a' as usize..])
            .enumerate()
            .take(ABC_LEN)
            .rev()
            .filter_map(|(index, (&upper, &lower))| if upper && lower { Some(index) } else { None })
            .next()
        {
            ((index as u8 + b'A') as char).into()
        } else {
            String::new()
        }
    }
}
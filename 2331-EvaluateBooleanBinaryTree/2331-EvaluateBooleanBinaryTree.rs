// Last updated: 08.09.2025, 16:22:29
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut seen = vec![false; b'z' as usize + 1];
        for byte in s.bytes() {
            let mut seen = &mut seen[byte as usize];
            if *seen {
                return byte as char;
            }
            *seen = true;
        }
        unimplemented!()
    }
}
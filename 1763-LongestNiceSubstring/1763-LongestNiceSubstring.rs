// Last updated: 19.06.2025, 15:48:15
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut seen = [false; b'z' as usize + 1];
        for byte in sentence.bytes() {
            seen[byte as usize] = true;
        }
        seen[b'a' as usize..].into_iter().all(|&seen| seen)
    }
}
// Last updated: 25.07.2025, 14:48:39
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn frequency(word: String) -> [u8; ABC_LEN] {
    let mut freq = [0; ABC_LEN];
    for byte in word.bytes() {
        freq[(byte - b'a') as usize] += 1;
    }
    freq
}

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        if word1.len() <= 3 {
            return true;
        }
        frequency(word1)
            .into_iter()
            .zip(frequency(word2))
            .all(|(freq1, freq2)| freq1.abs_diff(freq2) <= 3)
    }
}
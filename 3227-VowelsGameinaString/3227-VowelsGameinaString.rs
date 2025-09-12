// Last updated: 12.09.2025, 06:57:44
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const IS_VOWEL: [bool; ABC_LEN] = [
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false,
];

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.bytes().any(|byte| IS_VOWEL[(byte - b'a') as usize])
    }
}
// Last updated: 16.06.2025, 14:08:36
static IS_VOWEL: [bool; 123] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, true, false, false, false, true, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, true, false, false, false,
    false, false,
];

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let half = s.len() / 2;
        s.bytes()
            .take(half)
            .filter(|byte| IS_VOWEL[*byte as usize])
            .count()
            == s.bytes()
                .skip(half)
                .filter(|byte| IS_VOWEL[*byte as usize])
                .count()
    }
}
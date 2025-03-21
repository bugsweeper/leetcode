// Last updated: 21.03.2025, 16:32:41
const VOWELS: [u8; 10] = [b'A', b'E', b'I', b'O', b'U', b'a', b'e', b'i', b'o', b'u'];

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = std::collections::HashSet::from(VOWELS);
        let mut s = s;
        let s_bytes = unsafe { s.as_bytes_mut() };
        let mut vowels_position = Vec::with_capacity(s_bytes.len());
        for (index, character) in s_bytes.iter().enumerate() {
            if vowels.contains(character) {
                vowels_position.push(index);
            }
        }
        if vowels_position.len() > 1 {
            let end = vowels_position.len() - 1;
            for i in 0..=end / 2 {
                s_bytes.swap(vowels_position[i], vowels_position[end - i]);
            }
        }
        s
    }
}
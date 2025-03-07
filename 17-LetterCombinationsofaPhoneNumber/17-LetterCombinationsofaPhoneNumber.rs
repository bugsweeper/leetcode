const LETTERS: [&[u8]; 8] = [
    "abc".as_bytes(),
    "def".as_bytes(),
    "ghi".as_bytes(),
    "jkl".as_bytes(),
    "mno".as_bytes(),
    "pqrs".as_bytes(),
    "tuv".as_bytes(),
    "wxyz".as_bytes(),
];

fn combinate(source: &[u8], destination: &mut Vec<String>, buffer: &mut [u8], depth: usize) {
    for &letter in LETTERS[(source[depth] - b'2') as usize] {
        buffer[depth] = letter;
        if depth == buffer.len() - 1 {
            destination.push(unsafe { String::from_utf8_unchecked(buffer.to_vec()) });
        } else {
            combinate(source, destination, buffer, depth + 1);
        }
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let mut result = Vec::with_capacity(256);
        let mut buffer = vec![b'a'; digits.len()];
        combinate(digits.as_bytes(), &mut result, &mut buffer, 0);
        result
    }
}
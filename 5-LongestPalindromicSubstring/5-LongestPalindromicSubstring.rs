// Last updated: 18.04.2025, 16:11:06
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // Manacher's algorithm
        let mut row = Vec::with_capacity(s.len() * 2 + 1);
        row.push(b'#');
        for &c in s.as_bytes() {
            row.push(c);
            row.push(b'#');
        }
        // Radiuses of palindromes
        let mut palindromes = vec![0; row.len()];
        // Center of palindrome with most right position
        let mut center = 0;
        // Most right position for any known palindrome
        let mut radius = 0;
        for i in 0..row.len() {
            if i < radius {
                let mirror = 2 * center - i;
                // in case if we are inside palindrome we can copy already
                // defined mirrored value in palindrome
                palindromes[i] = (radius - i).min(palindromes[mirror]);
            }
            // Look for making palindrome wider
            let (mut right, mut after_left) = (i + palindromes[i] + 1, i - palindromes[i]);
            while right < row.len() && after_left > 0 && row[right] == row[after_left - 1] {
                palindromes[i] += 1;
                right += 1;
                after_left -= 1;
            }
            if i + palindromes[i] > radius {
                (center, radius) = (i, i + palindromes[i]);
            }
        }
        // here index also is maxed, if most left palindrome should be found
        // then Reverse(index) should be used
        let (max_len, center) = palindromes
            .into_iter()
            .enumerate()
            .map(|(index, value)| (value, index))
            .max()
            .unwrap();
        let start = (center - max_len) / 2;
        s[start..start + max_len].into()
    }
}
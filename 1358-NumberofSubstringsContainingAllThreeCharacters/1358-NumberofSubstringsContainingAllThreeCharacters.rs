impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut letter_count = vec![0; b'z' as usize + 1];
        let mut letters = 0;
        let mut result = 0;
        let mut left = 0;
        for right in 0..s.len() {
            let right_letter = s[right] as usize;
            let count = letter_count.get_mut(right_letter).unwrap();
            if *count == 0 {
                letters += 1;
            }
            *count += 1;
            
            while letters == 3 {
                result += s.len() - right;
                let left_letter = s[left] as usize;
                let count = letter_count.get_mut(left_letter).unwrap();
                *count -= 1;
                if *count == 0 {
                    letters -= 1;
                }
                left += 1;
            }
        }
        result as i32
    }
}
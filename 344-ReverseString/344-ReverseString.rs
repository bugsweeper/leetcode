impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let end = s.len() - 1;
        for i in 0..=end / 2 {
            s.swap(i, end - i);
        }
    }
}
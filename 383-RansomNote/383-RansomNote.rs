const abc_len: usize = (b'z' - b'a' + 1) as usize;

fn get_statistic(symbols: &[u8]) -> [usize; abc_len] {
    let mut result = [0; abc_len];
    for &symbol in symbols {
        result[(symbol - b'a') as usize] += 1;
    }
    result
}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        get_statistic(ransom_note.as_bytes()).iter().zip(get_statistic(magazine.as_bytes()).iter()).all(|(ransom_note, magazine)| ransom_note <= magazine)
    }
}
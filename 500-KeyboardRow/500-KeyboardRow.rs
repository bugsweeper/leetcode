// Last updated: 24.03.2025, 12:08:38
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut dict = [0; ABC_LEN];
        for (mut index, row) in ["asdfghjkl", "zxcvbnm"].iter().enumerate() {
            index += 1;
            for letter in row.as_bytes() {
                dict[(letter - b'a') as usize] = index;
            }
        }
        words
            .into_iter()
            .filter(|word| {
                let mut iter = word.as_bytes().iter();
                let row_index = dict[(iter.next().unwrap().to_ascii_lowercase() - b'a') as usize];
                iter.all(|letter| row_index == dict[(letter.to_ascii_lowercase() - b'a') as usize])
            })
            .collect()
    }
}
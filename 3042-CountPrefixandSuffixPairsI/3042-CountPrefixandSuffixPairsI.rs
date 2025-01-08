impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut counter = 0;
        for (index, str1) in words[..words.len() - 1].iter().enumerate() {
            for str2 in &words[index + 1..] {
                if str1.len() > str2.len() {
                    continue;
                }
                if str2.starts_with(str1) && str2.ends_with(str1) {
                    counter += 1;
                }
            }
        }
        counter
    }
}
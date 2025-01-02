fn is_vowel(c: u8) -> bool {
    c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u'
}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut vowel_words_counter = 0;
        let mut vowel_words_count = Vec::with_capacity(words.len() + 1);
        vowel_words_count.push(0);
        for word in words {
            let word = word.as_bytes();
            if unsafe{ is_vowel(*word.get_unchecked(0)) && is_vowel(*word.get_unchecked(word.len() - 1)) } {
                vowel_words_counter += 1;
            }
            vowel_words_count.push(vowel_words_counter);
        }
        queries.iter().map(|query| {
            ( unsafe{ vowel_words_count[*query.get_unchecked(1) as usize + 1] - vowel_words_count[*query.get_unchecked(0) as usize] } ) as i32
        }).collect::<Vec<_>>()
    }
}
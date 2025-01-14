const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut l2w = [""; ABC_LEN];
        let mut w2l = std::collections::HashMap::with_capacity(ABC_LEN);
        let mut pattern_iter = pattern.as_bytes().iter();
        let mut word_iter = s.split(' ');
        loop {
            match (pattern_iter.next(), word_iter.next()) {
                (Some(&letter), Some(word)) => {
                    let letter_index = (letter - b'a') as usize;
                    let assigned_word = unsafe { l2w.get_unchecked_mut(letter_index) };
                    if assigned_word.is_empty() {
                        *assigned_word = word;
                    } else if *assigned_word != word {
                        return false;
                    }
                    if let Some(previous_letter) = w2l.insert(word, letter) {
                        if previous_letter != letter {
                            return false;
                        }
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
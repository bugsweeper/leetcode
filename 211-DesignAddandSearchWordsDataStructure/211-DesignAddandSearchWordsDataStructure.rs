// Last updated: 25.03.2025, 11:22:10
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[derive(Default)]
struct WordDictionary {
    equals: bool,
    continues: [Option<Box<WordDictionary>>; ABC_LEN],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            equals: false,
            continues: [const { None }; ABC_LEN],
        }
    }

    fn add_word(&mut self, word: String) {
        self.add_word_sequence(word.as_bytes().iter());
    }

    fn search(&self, word: String) -> bool {
        self.search_slice(word.as_bytes())
    }

    fn add_word_sequence(&mut self, mut word_sequence: std::slice::Iter<'_, u8>) {
        if let Some(&first) = word_sequence.next() {
            let index = (first - b'a') as usize;
            let sub_dictionary = self.continues.get_mut(index).unwrap();
            if sub_dictionary.is_none() {
                let mut new_sub_dictionary: Box<WordDictionary> = Box::default();
                new_sub_dictionary.add_word_sequence(word_sequence);
                *sub_dictionary = Some(new_sub_dictionary);
            } else {
                sub_dictionary
                    .as_mut()
                    .unwrap()
                    .add_word_sequence(word_sequence);
            }
        } else {
            self.equals = true;
        }
    }

    fn search_slice(&self, word_slice: &[u8]) -> bool {
        if let Some(&first) = word_slice.first() {
            let word_sub_slice = &word_slice[1..];
            if first == b'.' {
                self.continues.iter().any(|cell| {
                    cell.is_some() && cell.as_ref().unwrap().search_slice(word_sub_slice)
                })
            } else {
                let index = (first - b'a') as usize;
                let cell = self.continues.get(index).unwrap();
                cell.is_some() && cell.as_ref().unwrap().search_slice(word_sub_slice)
            }
        } else {
            self.equals
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
// Last updated: 24.03.2025, 10:43:56
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[derive(Clone)]
struct Trie {
    contains: bool,
    continues: Vec<Option<Trie>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            contains: false,
            continues: Vec::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let iter = word.as_bytes().iter();
        self.insert_sequence(iter);
    }

    fn search(&self, word: String) -> bool {
        let iter = word.as_bytes().iter();
        self.search_sequence(iter)
    }

    fn starts_with(&self, prefix: String) -> bool {
        let iter = prefix.as_bytes().iter();
        self.starts_with_sequence(iter)
    }

    fn insert_sequence(&mut self, mut sequence: std::slice::Iter<'_, u8>) {
        if let Some(first) = sequence.next() {
            let index = (first - b'a') as usize;
            if self.continues.is_empty() {
                self.continues = vec![None; ABC_LEN];
                let mut sub_trie = Self::new();
                sub_trie.insert_sequence(sequence);
                self.continues[index] = Some(sub_trie);
            } else if let Some(Some(sub_trie)) = self.continues.get_mut(index) {
                sub_trie.insert_sequence(sequence);
            } else {
                let mut sub_trie = Self::new();
                sub_trie.insert_sequence(sequence);
                self.continues[index] = Some(sub_trie);
            }
        } else {
            self.contains = true;
        }
    }

    fn search_sequence(&self, mut sequence: std::slice::Iter<'_, u8>) -> bool {
        if let Some(first) = sequence.next() {
            let index = (first - b'a') as usize;
            if let Some(Some(sub_trie)) = self.continues.get(index) {
                sub_trie.search_sequence(sequence)
            } else {
                false
            }
        } else {
            self.contains
        }
    }

    fn starts_with_sequence(&self, mut sequence: std::slice::Iter<'_, u8>) -> bool {
        if let Some(first) = sequence.next() {
            let index = (first - b'a') as usize;
            if let Some(Some(sub_trie)) = self.continues.get(index) {
                sub_trie.starts_with_sequence(sequence)
            } else {
                false
            }
        } else {
            self.contains || self.continues.iter().any(|sub_trie| sub_trie.is_some())
        }
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
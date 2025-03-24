// Last updated: 24.03.2025, 10:45:01
struct Trie {
    node: Node,
}

impl Trie {
    fn new() -> Self {
        Self {
            node: Default::default(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.node;
        for c in word.bytes() {
            let i = (c - b'a') as usize;
            cur = cur.children[i].get_or_insert_with(|| Box::new(Node::default()));
        }
        cur.mark = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = &self.node;
        for c in word.bytes() {
            match cur.children[(c - b'a') as usize].as_ref() {
                Some(child) => cur = child,
                None => return false,
            }
        }
        cur.mark
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.node;
        for c in prefix.bytes() {
            match cur.children[(c - b'a') as usize].as_ref() {
                Some(child) => cur = child,
                None => return false,
            }
        }
        true
    }
}

#[derive(Default)]
struct Node {
    mark: bool,
    children: [Option<Box<Node>>; 26],
}
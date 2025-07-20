// Last updated: 20.07.2025, 20:40:01
use std::collections::{HashMap, BTreeMap};
use std::hash::{DefaultHasher, Hasher};

#[derive(Debug)]
struct Trie<'a> {
    hash: u64,
    subfolders: BTreeMap<&'a str, Trie<'a>>,
}

impl<'a> Trie<'a> {
    fn new() -> Self {
        Self {
            hash: 0,
            subfolders: BTreeMap::new(),
        }
    }
    fn add(&mut self, path: &'a [String]) {
        if let Some((subfolder_name, subpath)) = path.split_first() {
            if let Some(subfolder) = self.subfolders.get_mut(subfolder_name.as_str()) {
                subfolder.add(subpath);
                return;
            }
            let mut subfolder = Trie::new();
            subfolder.add(subpath);
            self.subfolders.insert(subfolder_name, subfolder);
        }
    }
    fn calc_hash(&mut self, hashes: &mut HashMap<u64, i32>) {
        let mut hasher = DefaultHasher::new();
        for (&subfolder_name, trie) in &mut self.subfolders {
            hasher.write(subfolder_name.as_bytes());
            hasher.write_u8(0xff);
            trie.calc_hash(hashes);
            hasher.write_u64(trie.hash);
        }
        hasher.write_u16(0xffff);
        self.hash = hasher.finish();
        hashes
            .entry(self.hash)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    fn is_unique(&self, path: &[String], hashes: &HashMap<u64, i32>) -> bool {
        if self.subfolders.is_empty() {
            return true;
        }
        if *hashes.get(&self.hash).unwrap() > 1 {
            return false;
        }
        if let Some((subfolder_name, subpath)) = path.split_first() {
            return self
                .subfolders
                .get(subfolder_name.as_str())
                .unwrap()
                .is_unique(subpath, hashes);
        }
        true
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for path in &paths {
            trie.add(path);
        }
        let mut hashes = HashMap::new();
        trie.calc_hash(&mut hashes);
        paths
            .iter()
            .filter_map(|path| {
                if trie.is_unique(path, &hashes) {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
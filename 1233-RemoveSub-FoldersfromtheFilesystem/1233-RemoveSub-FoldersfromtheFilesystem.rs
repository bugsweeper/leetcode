// Last updated: 19.07.2025, 14:46:33
use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Eq)]
struct StrWithHash<'str> {
    id: u64,
    data: Cow<'str, str>,
}

impl Hash for StrWithHash<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for StrWithHash<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort_unstable_by_key(|folder| folder.len());
        let mut folders_set = HashSet::with_capacity(folder.len());
        'folder: for folder in folder {
            let mut hasher = DefaultHasher::new();
            let mut path_len = 0;
            for subfolder in folder.split('/').skip(1) {
                // The idea is to prevent from hashing from the start of the path for each subpath
                hasher.write(subfolder.as_bytes());
                hasher.write_u8(0xff);
                path_len += subfolder.len() + 1;
                if folders_set.contains(&StrWithHash{id: hasher.finish(), data: folder[..path_len].into()}) {
                    continue 'folder;
                }
            }
            folders_set.insert(StrWithHash{ id: hasher.finish(), data: folder.into()});
        }
        folders_set
            .into_iter()
            .map(|item| item.data.into_owned())
            .collect()
    }
}
use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .map(|tree_map| {
                tree_map
                    .range(..=timestamp)
                    .next_back()
                    .map(|(_, value)| value.clone())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
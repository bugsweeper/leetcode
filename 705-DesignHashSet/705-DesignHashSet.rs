// Last updated: 23.04.2025, 15:44:07
struct MyHashSet {
    buckets: Vec<Vec<i32>>,
    length: usize,
    mask: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            buckets: vec![vec![]; 8],
            length: 0,
            mask: 7,
        }
    }

    fn add(&mut self, key: i32) {
        if self.length * 2 > self.mask {
            self.resize();
        }
        let bucket = self.buckets.get_mut(key as usize & self.mask).unwrap();
        if !bucket.contains(&key) {
            bucket.push(key);
            self.length += 1;
        }
    }

    fn remove(&mut self, key: i32) {
        let bucket = self.buckets.get_mut(key as usize & self.mask).unwrap();
        if let Some(index) = bucket.iter_mut().position(|item| *item == key) {
            bucket.swap_remove(index);
            self.length -= 1;
        }
    }

    fn resize(&mut self) {
        let old_len = self.buckets.len();
        self.buckets.resize_with(old_len * 2, Default::default);
        self.mask = (self.mask << 1) + 1;
        let (from, to) = self.buckets.split_at_mut(old_len);
        for (index, (lower, higher)) in from.iter_mut().zip(to.iter_mut()).enumerate() {
            let mut old_source = Vec::with_capacity(lower.len());
            std::mem::swap(lower, &mut old_source);
            for item in old_source {
                if item as usize & self.mask == index {
                    lower.push(item);
                } else {
                    higher.push(item);
                }
            }
        }
    }

    fn contains(&self, key: i32) -> bool {
        self.buckets
            .get(key as usize & self.mask)
            .unwrap()
            .contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
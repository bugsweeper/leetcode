// Last updated: 16.06.2025, 11:00:10
struct OrderedStream {
    data: Vec<String>,
    ptr: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        Self {
            data: vec![String::new(); n as usize + 1],
            ptr: 1,
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id_key = id_key as usize;
        self.data[id_key] = value;
        let result = self.data[self.ptr..].iter().take_while(|value| value.len() == 5).cloned().collect::<Vec<_>>();
        self.ptr += result.len();
        result
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */
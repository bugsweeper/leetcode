struct SnapshotArray {
    elements: Vec<Vec<(i32, i32)>>,
    current_snap_id: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        Self {
            elements: vec![vec![(0, 0)]; length as usize],
            current_snap_id: 0,
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        let element = self.elements.get_mut(index as usize).unwrap();
        let last = element.last_mut().unwrap();
        if last.0 == self.current_snap_id {
            last.1 = val;
            return;
        }
        element.push((self.current_snap_id, val));
    }
    
    fn snap(&mut self) -> i32 {
        self.current_snap_id += 1;
        self.current_snap_id - 1
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let element = self.elements.get(index as usize).unwrap();
        let value_index = element.partition_point(|(value_snap_id, _)| *value_snap_id <= snap_id) - 1;
        element[value_index].1
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
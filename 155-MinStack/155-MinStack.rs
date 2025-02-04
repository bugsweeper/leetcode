struct MinStack {
    values: Vec<i32>,
    descending_values_position: Vec<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            descending_values_position: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.descending_values_position.is_empty() || self.get_min() > val {
            self.descending_values_position.push(self.values.len());
        }
        self.values.push(val);
    }

    fn pop(&mut self) {
        self.values.pop();
        if *unsafe { self.descending_values_position.last().unwrap_unchecked() }
            == self.values.len()
        {
            self.descending_values_position.pop();
        }
    }

    fn top(&self) -> i32 {
        *unsafe { self.values.last().unwrap_unchecked() }
    }

    fn get_min(&self) -> i32 {
        *unsafe {
            self.values
                .get_unchecked(*self.descending_values_position.last().unwrap_unchecked())
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
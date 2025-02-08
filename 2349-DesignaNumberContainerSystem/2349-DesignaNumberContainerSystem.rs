use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    number2indexes: HashMap<i32, BTreeSet<i32>>,
    index2number: HashMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            number2indexes: HashMap::new(),
            index2number: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(prev_number) = self.index2number.get_mut(&index) {
            self.number2indexes
                .get_mut(&prev_number)
                .unwrap()
                .remove(&index);
            *prev_number = number;
        } else {
            self.index2number.insert(index, number);
        }
        self.number2indexes.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(set) = self.number2indexes.get(&number) {
            return *set.first().unwrap_or(&-1);
        }
        -1
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
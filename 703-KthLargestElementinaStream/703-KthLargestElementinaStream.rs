// Last updated: 21.04.2025, 12:43:51
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    result: i32,
    top: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut top = BinaryHeap::with_capacity(nums.len().max(k));
        top.extend(nums.into_iter().map(Reverse));
        while top.len() > k {
            let _ = top.pop();
        }
        Self {
            k,
            result: top.peek().copied().unwrap_or(Reverse(i32::MIN)).0,
            top,
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        if self.top.len() < self.k {
            self.top.push(Reverse(val));
            self.result = self.top.peek().unwrap().0;
        } else if val > self.result {
            self.top.push(Reverse(val));
            let _ = self.top.pop();
            self.result = self.top.peek().unwrap().0;
        }
        self.result
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
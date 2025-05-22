// Last updated: 22.05.2025, 13:39:57
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Default)]
struct MedianFinder {
    lower: BinaryHeap<i32>,
    upper: BinaryHeap<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Default::default()
    }
    
    fn add_num(&mut self, num: i32) {
        if self.upper.is_empty() || num >= -*self.upper.peek().unwrap() {
            self.upper.push(-num);
            if self.upper.len() > self.lower.len() + 1 {
                let val = self.upper.pop().unwrap();
                self.lower.push(-val);
            }
        } else {
            self.lower.push(num);
            if self.lower.len() > self.upper.len() + 1 {
                let val = self.lower.pop().unwrap();
                self.upper.push(-val);
            }
        }
    }
    
    fn find_median(&self) -> f64 {
        match self.lower.len().cmp(&self.upper.len()) {
            Ordering::Less => {
                -*self.upper.peek().unwrap() as f64
            },
            Ordering::Equal => {
                let lower_max = *self.lower.peek().unwrap();
                let upper_min = -*self.upper.peek().unwrap();
                (lower_max + upper_min) as f64 / 2.0
            },
            Ordering::Greater => {
                *self.lower.peek().unwrap() as f64
            }
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
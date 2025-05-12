// Last updated: 12.05.2025, 22:33:17
use std::collections::VecDeque;

struct RecentCounter {
    requests: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self {
            requests: VecDeque::with_capacity(10_000)
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push_back(t);
        let lower_limit = t - 3000;
        while *self.requests.front().unwrap() < t - 3000 {
            self.requests.pop_front();
        }
        self.requests.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// Last updated: 30.05.2025, 12:08:47
use rand::Rng;

struct Solution {
    sum: i32,
    weighted_positions: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let mut sum = 0;
        let mut weighted_positions = Vec::with_capacity(w.len());
        for weight in w {
            sum += weight;
            weighted_positions.push(sum);
        }
        Self { sum, weighted_positions }
    }
    
    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut position = rng.gen_range(0..self.sum);
        self.weighted_positions.partition_point(|&weighted_position| weighted_position <= position) as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
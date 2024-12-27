#[derive(Clone, Copy)]
struct Spot {
    value: i32,
    index: i32,
}

impl Spot {
    fn new(value: i32, index: usize) -> Self {
        Self {
            value,
            index: index as i32,
        }
    }
    #[inline]
    fn start(&self) -> i32 {
        self.value + self.index
    }
    #[inline]
    fn end(&self) -> i32 {
        self.value - self.index
    }
}

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut current_start_spot = Spot::new(values[0], 0);
        let mut current_end_spot = Spot::new(values[1], 1);
        let mut max_sum = current_start_spot.start() + current_end_spot.end();
        let mut max_start_spot = if current_start_spot.start() > current_end_spot.start() {
            current_start_spot
        } else {
            current_end_spot
        };
        for (index, value) in values.into_iter().enumerate().skip(2) {
            let third_spot = Spot::new(value, index);
            if max_start_spot.start() + third_spot.end() > max_sum {
                current_start_spot = max_start_spot;
                current_end_spot = third_spot;
                max_sum = current_start_spot.start() + current_end_spot.end();
            }
            if max_start_spot.start() < third_spot.start() {
                max_start_spot = third_spot;
            }
        }
        current_start_spot.start() + current_end_spot.end()
    }
}
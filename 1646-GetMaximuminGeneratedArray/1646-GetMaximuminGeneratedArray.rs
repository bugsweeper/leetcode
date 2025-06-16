// Last updated: 16.06.2025, 15:24:42
use std::cmp::Ordering;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_side = 0;
        let mut count = 0;
        for rectangle in rectangles {
            let side = rectangle[0].min(rectangle[1]);
            match side.cmp(&max_side) {
                Ordering::Greater => {
                    max_side = side;
                    count = 1;
                }
                Ordering::Equal => count += 1,
                _ => {}
            }
        }
        count
    }
}
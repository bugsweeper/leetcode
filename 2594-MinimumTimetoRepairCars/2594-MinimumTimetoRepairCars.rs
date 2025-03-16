impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut min = i32::MAX;
        for rank in &ranks {
            min = min.min(*rank);
        }
        let cars = cars as i64;
        let mut base = 0;
        let mut size = (min as i64).saturating_mul(cars).saturating_mul(cars).saturating_add(1);
        while size > 1 {
            let half = size / 2;
            let middle_time = base + half;
            let mut cars_washed = 0;
            for rank in &ranks {
                cars_washed += (middle_time / *rank as i64).isqrt();
                if cars_washed >= cars {
                    break;
                }
            }
            if cars_washed < cars {
                base = middle_time;
            }
            size -= half;
        }
        base + 1
    }
}
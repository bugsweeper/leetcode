// Last updated: 03.05.2025, 11:35:18
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut numbers_left = 6;
        let mut available = [true; 7];
        let mut top_statistic = [0; 7];
        let mut bottom_statistic = [0; 7];
        let n = tops.len() as i32;
        for (top, bottom) in tops.into_iter().zip(bottoms) {
            for number in 1..=6 {
                let available = available.get_mut(number as usize).unwrap();
                if !*available {
                    continue;
                }
                if number != top && number != bottom {
                    numbers_left -= 1;
                    if numbers_left == 0 {
                        return -1;
                    }
                    *available = false;
                    continue;
                }
                if number == top {
                    top_statistic[number as usize] += 1;
                }
                if number == bottom {
                    bottom_statistic[number as usize] += 1;
                }
            }
        }
        let mut min_rotations = n;
        for ((top, bottom), available) in top_statistic.into_iter().zip(bottom_statistic.into_iter()).zip(available.into_iter()).skip(1) {
            if available {
                min_rotations = min_rotations.min(n - top.max(bottom));
            }
        }
        min_rotations
    }
}
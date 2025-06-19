// Last updated: 19.06.2025, 14:28:58
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .into_iter()
            .enumerate()
            .fold((i32::MAX, -1), |(min_distance, min_index), (index, point)| {
                let (a, b) = (point[0], point[1]);
                if a == x {
                    let distance = (b - y).abs();
                    if min_distance > distance {
                        return (distance, index as i32);
                    }
                } else if b == y {
                    let distance = (a - x).abs();
                    if min_distance > distance {
                        return (distance, index as i32);
                    }
                }
                (min_distance, min_index)
            }).1
    }
}
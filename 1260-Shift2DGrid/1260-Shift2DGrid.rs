// Last updated: 22.05.2025, 14:12:41
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let &[mut x_prev, mut y_prev] = &points.first().unwrap()[..] else {
            unimplemented!();
        };
        let mut time = 0;
        for point in points.into_iter().skip(1) {
            let &[x, y] = &point[..] else {
                unimplemented!();
            };
            time += (x - x_prev).abs().max((y - y_prev).abs());
            (x_prev, y_prev) = (x, y)
        }
        time
    }
}
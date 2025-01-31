impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable();
        let mut arrows = 1;
        let mut current_end = points[0][1];
        for balloon in points {
            let [start, end] = balloon[..] else {
                continue;
            };
            if start > current_end {
                arrows += 1;
                current_end = end;
                continue;
            }
            current_end = current_end.min(end)
        }
        arrows
    }
}
// Last updated: 03.09.2025, 07:48:33
impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points
            .into_iter()
            .map(|point| (point[0], -point[1]))
            .collect::<Vec<_>>();
        points.sort_unstable();
        let mut pairs = 0;
        for (i, &(_, yi)) in points.iter().enumerate() {
            let mut upper_limit = i32::MAX;
            for &(_, yj) in &points[i + 1..] {
                if yj < yi {
                    continue;
                }
                if yj < upper_limit {
                    pairs += 1;
                    upper_limit = yj;
                }
            }
        }
        pairs
    }
}
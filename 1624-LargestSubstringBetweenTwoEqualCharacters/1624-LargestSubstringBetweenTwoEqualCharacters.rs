// Last updated: 12.06.2025, 16:15:41
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut abscissa = points.into_iter().map(|point| point[0]).collect::<Vec<_>>();
        abscissa.sort_unstable();
        let mut iter = abscissa.into_iter();
        let mut prev_x = iter.next().unwrap();
        let mut max_width = 0;
        for x in iter {
            if prev_x != x {
                max_width = max_width.max(x - prev_x);
                prev_x = x;
            }
        }
        max_width
    }
}
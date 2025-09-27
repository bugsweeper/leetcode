// Last updated: 27.09.2025, 15:30:27
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut area = 0;
        for (i1, p1) in points.iter().enumerate() {
            let [x1, y1] = p1[..] else {
                unimplemented!()
            };
            for (i2, p2) in points.iter().enumerate().skip(i1 + 1) {
                let [x2, y2] = p2[..] else {
                    unimplemented!()
                };
                let (v1x, v1y) = (x1 - x2, y1 - y2);
                for p3 in points.iter().skip(i2 + 1) {
                    let [x3, y3] = p3[..] else {
                        unimplemented!()
                    };
                    let (v2x, v2y) = (x1 - x3, y1 - y3);
                    area = area.max((v1x * v2y - v1y * v2x).abs())
                }
            }
        }
        area as f64 / 2.0
    }
}
// Last updated: 21.05.2025, 16:31:52
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut iter = coordinates.into_iter();
        let [x0, y0] = iter.next().unwrap()[..] else {
            unimplemented!();
        };
        let [x1, y1] = iter.next().unwrap()[..] else {
            unimplemented!();
        };
        let (dx0, dy0) = (x1 - x0, y1 - y0);
        for slice in iter {
            let [x, y] = slice[..] else {
                unimplemented!();
            };
            if (x - x0) * dy0 != dx0 * (y - y0) {
                return false;
            }
        }
        true
    }
}
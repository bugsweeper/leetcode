// Last updated: 25.03.2025, 09:56:43
impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let (mut range_x, mut range_y): (Vec<_>, Vec<_>) = rectangles.iter().map(|rectangle| {
            let [start_x, start_y, end_x, end_y] = rectangle[..] else {
                unimplemented!();
            };
            ((start_x, end_x), (start_y, end_y))
        }).unzip();
        for mut range in [range_x, range_y] {
            range.sort_unstable_by_key(|(start, _)| *start);
            let mut range_iter = range.iter();
            let mut max_end = range_iter.next().unwrap().1;
            let mut cuts = 0;
            for &(start, end) in range_iter {
                if start >= max_end {
                    cuts += 1;
                    if cuts >= 2 {
                        return true;
                    }
                }
                max_end = max_end.max(end);
            }
        }
        false
    }
}
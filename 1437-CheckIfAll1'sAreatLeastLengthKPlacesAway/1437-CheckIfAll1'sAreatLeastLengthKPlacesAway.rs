// Last updated: 03.06.2025, 13:47:59
use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut point_set = HashSet::with_capacity(path.len());
        let (mut x, mut y) = (0, 0);
        point_set.insert((x, y));
        for &direction in path.as_bytes() {
            match direction {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                b'W' => x -= 1,
                _ => unimplemented!(),
            }
            if !point_set.insert((x, y)) {
                return true;
            }
        }
        false
    }
}
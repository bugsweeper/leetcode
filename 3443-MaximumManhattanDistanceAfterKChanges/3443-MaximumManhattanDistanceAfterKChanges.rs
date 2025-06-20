// Last updated: 20.06.2025, 12:20:18
const N: usize = b'N' as usize;
const S: usize = b'S' as usize;
const E: usize = b'E' as usize;
const W: usize = b'W' as usize;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut directions = [
            ([false, true, false, true], 0, k),
            ([false, true, true, false], 0, k),
            ([true, false, false, true], 0, k),
            ([true, false, true, false], 0, k),
        ];
        let mut index = [0; W + 1];
        index[S] = 1;
        index[E] = 2;
        index[W] = 3;
        let mut max_distance = 0;
        for byte in s.bytes() {
            let index = index[byte as usize];
            for (desired, distance, inversions) in &mut directions {
                if desired[index] {
                    *distance += 1;
                } else if *inversions > 0 {
                    *inversions -= 1;
                    *distance += 1;
                } else {
                    *distance -= 1;
                }
                max_distance = max_distance.max(*distance);
            }
        }
        max_distance
    }
}
// Last updated: 04.05.2025, 08:03:51
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut domino_count = vec![0; 81];
        for domino in dominoes {
            let [mut a, mut b] = domino[..] else {
                unimplemented!();
            };
            if a > b {
                (a, b) = (b, a);
            }
            domino_count[((a - 1) * 9 + b - 1) as usize] += 1;
        }
        let mut count = 0;
        for domino_count in domino_count {
            if domino_count > 0 {
                count += domino_count * (domino_count - 1) / 2;
            }
        }
        count
    }
}
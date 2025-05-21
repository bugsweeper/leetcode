// Last updated: 21.05.2025, 15:05:37
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut even, mut odd) = (0, 0);
        for position in position {
            if position & 1 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        odd.min(even)
    }
}
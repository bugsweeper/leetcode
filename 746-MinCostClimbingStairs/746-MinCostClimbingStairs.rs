// Last updated: 24.04.2025, 15:05:45
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut iter = cost.into_iter();
        let (mut cost1, mut cost2) = (iter.next().unwrap(), iter.next().unwrap());
        for cost in iter {
            (cost1, cost2) = (cost2, cost + cost1.min(cost2));
        }
        cost1.min(cost2)
    }
}
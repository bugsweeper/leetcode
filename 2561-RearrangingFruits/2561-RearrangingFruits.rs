// Last updated: 02.08.2025, 17:03:04
use std::collections::BTreeMap;

// Intuition: calculate diff between baskets, starting from lower costs
// (that is why we need ordered map cost->count) add to result half of diff count * cost,
// till we met half of count
impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut fruits_cost2diff_count: BTreeMap<i32, i32> = BTreeMap::new();
        for fruit_cost1 in basket1 {
            fruits_cost2diff_count
                .entry(fruit_cost1)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for fruit_cost2 in basket2 {
            fruits_cost2diff_count
                .entry(fruit_cost2)
                .and_modify(|count| *count -= 1)
                .or_insert(-1);
        }
        let mut diff_count_sum = 0;
        for count in fruits_cost2diff_count.values_mut() {
            *count = count.abs();
            if *count & 1 == 1 {
                return -1;
            }
            diff_count_sum += *count as i64;
        }
        let mut half_diff_count_sum = diff_count_sum >> 1;
        let min_fruit_cost = *fruits_cost2diff_count.keys().next().unwrap();
        let mut answer = 0;
        for (&fruit_cost, &diff_count) in fruits_cost2diff_count.range(..(min_fruit_cost << 1)) {
            if diff_count == 0 {
                continue;
            }
            if half_diff_count_sum > 0 {
                let mut diff_count = diff_count as i64;
                diff_count = diff_count.min(half_diff_count_sum);
                answer += (diff_count >> 1) * fruit_cost as i64;
                half_diff_count_sum -= diff_count;
            } else {
                break;
            }
        }
        if half_diff_count_sum > 0 {
            answer += half_diff_count_sum * min_fruit_cost as i64;
        }
        answer
    }
}
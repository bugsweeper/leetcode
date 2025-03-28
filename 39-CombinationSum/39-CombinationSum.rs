// Last updated: 28.03.2025, 14:51:17
fn find_combinations(candidates: &Vec<i32>, mut max_index: usize, mut target: i32, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    while max_index > 0 && candidates[max_index] > target {
        max_index -= 1;
    }
    if candidates[max_index] > target {
        return;
    }
    let combination_size = combination.len();
    for i in (0..=max_index).rev() {
        let candidate = candidates[i];
        combination.push(candidate);
        let next_target = target - candidate;
        if next_target == 0 {
            result.push(combination.clone());
        } else {
            find_combinations(&candidates, i, target - candidate, combination, result);
        }
        combination.truncate(combination_size);
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        let mut result = Vec::with_capacity(150);
        let mut combination = Vec::with_capacity(20);
        find_combinations(&candidates, candidates.len() - 1, target, &mut combination, &mut result);
        result
    }
}
// Last updated: 29.09.2025, 10:30:52
pub fn min_score_triangulation(
    values: &[i32],
    start: usize,
    end: usize,
    dp: &mut Vec<Option<i32>>,
) -> i32 {
    let dp_index = start * values.len() + end;
    if let Some(score) = dp[dp_index] {
        return score;
    }
    if end - start == 1 {
        return 0;
    }
    let side_weight = values[start] * values[end];
    let mut min_score = i32::MAX;
    for middle in start + 1..end {
        min_score = min_score.min(
            min_score_triangulation(values, start, middle, dp)
                + side_weight * values[middle]
                + min_score_triangulation(values, middle, end, dp),
        );
    }
    dp[dp_index] = Some(min_score);
    min_score
}

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut dp = vec![None; (values.len() - 1) * values.len()];
        for i in 2..values.len() {
            dp[(i - 2) * values.len() + i] = Some(values[i - 2] * values[i - 1] * values[i]);
        }
        min_score_triangulation(&values, 0, values.len() - 1, &mut dp)
    }
}
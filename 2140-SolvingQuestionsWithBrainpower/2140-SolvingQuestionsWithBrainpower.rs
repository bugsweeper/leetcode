// Last updated: 01.04.2025, 10:47:34
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut maximum_points = vec![0; questions.len()];
        for (index, question) in questions.into_iter().enumerate().rev() {
            let [points, brainpower] = question[..] else {
                continue;
            };
            let brainpower = brainpower as usize;
            let points = points as i64;
            maximum_points[index] = (points
                + maximum_points
                    .get(index + brainpower + 1)
                    .copied()
                    .unwrap_or(0))
            .max(maximum_points.get(index + 1).copied().unwrap_or(0));
        }
        maximum_points[0]
    }
}
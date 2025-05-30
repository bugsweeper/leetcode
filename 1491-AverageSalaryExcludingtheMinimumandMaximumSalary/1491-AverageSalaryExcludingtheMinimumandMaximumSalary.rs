// Last updated: 30.05.2025, 12:19:41
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let count = (salary.len() - 2) as f64;
        let (sum, min, max) = salary
            .into_iter()
            .fold((0, i32::MAX, i32::MIN), |(sum, min, max), salary| {
                (sum + salary, min.min(salary), max.max(salary))
            });
        (sum - min - max) as f64 / count
    }
}
// Last updated: 10.06.2025, 11:49:31
impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort_unstable();
        let cut_count = arr.len() / 20;
        arr[cut_count..arr.len() - cut_count].iter().sum::<i32>() as f64 / (arr.len() - 2 * cut_count) as f64
    }
}
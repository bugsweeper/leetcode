impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 1..numbers.len() {
            if let Ok(pair_index) = numbers[0..i].binary_search(&(target - numbers[i])) {
                return vec![pair_index as i32 + 1, i as i32 + 1];
            }
        }
        vec![]
    }
}
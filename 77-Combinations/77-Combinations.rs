// Last updated: 26.03.2025, 14:15:26
fn fill_cell(index: usize, from: i32, n: i32, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let till = n - combination.len() as i32 + index as i32 + 1;
    if index == combination.len() - 1 {
        for value in from..=till {
            combination[index] = value;
            result.push(combination.clone());
        }
    } else {
        let next_index = index + 1;
        for value in from..=till {
            combination[index] = value;
            fill_cell(next_index, value + 1, n, combination, result);
        }
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combination = vec![1; k as usize];
        let (n_k, mut f, mut f_k, mut f_n_k) = (n - k, 1, 1, 1);
        for i in 1..=n {
            f *= i as usize;
            if i == n_k {
                f_n_k = f;
            }
            if i == k {
                f_k = f;
            }
        }
        let mut result = Vec::with_capacity(f / f_n_k / f_k);
        fill_cell(0, 1, n, &mut combination, &mut result);
        result
    }
}
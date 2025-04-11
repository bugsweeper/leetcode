// Last updated: 11.04.2025, 15:54:51
use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let list: HashMap<_, _> = list1
            .into_iter()
            .enumerate()
            .map(|(index, value)| (value, index))
            .collect();
        let mut least_sum = usize::MAX;
        let mut result = Vec::with_capacity(list2.len());
        for (index2, value) in list2.into_iter().enumerate() {
            let Some(&index1) = list.get(&value) else {
                continue;
            };
            let sum = index1 + index2;
            match sum.cmp(&least_sum) {
                Ordering::Equal => result.push(value),
                Ordering::Less => {
                    least_sum = sum;
                    result.clear();
                    result.push(value);
                }
                _ => {}
            }
        }
        result
    }
}
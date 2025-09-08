// Last updated: 08.09.2025, 16:40:35
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut weights = vec![0; 1001];
        for items in [items1, items2] {
            for item in items {
                weights[item[0] as usize] += item[1];
            }
        }
        weights
            .into_iter()
            .enumerate()
            .filter_map(|(index, weight)| {
                if weight > 0 {
                    Some(vec![index as i32, weight])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
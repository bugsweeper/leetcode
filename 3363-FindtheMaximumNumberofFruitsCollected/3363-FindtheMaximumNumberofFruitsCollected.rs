// Last updated: 07.08.2025, 11:38:31
impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        // We will destroy input data, but will not make any allocations
        let mut fruits = fruits;
        let diagonal = fruits
            .iter()
            .enumerate()
            .map(|(index, row)| row[index])
            .sum::<i32>();
        // side children are searching max till diagonal
        let last_index = fruits.len() - 1;
        let mut prev_min_position = last_index;
        for step in 1..last_index {
            let prev_step = step - 1;
            let distance_from_end = last_index - step;
            let cur_min_position = fruits.len() - distance_from_end.min(step + 1);
            // Iterate through possible positions at each step,
            // for top right child this is j,
            // for bottom left child this is i
            for position in cur_min_position..fruits.len() {
                let from = prev_min_position.max(position - 1);
                let to = last_index.min(position + 1);
                // top right child
                fruits[step][position] +=
                    fruits[prev_step][from..=to].iter().copied().max().unwrap();
                // bottom left child
                fruits[position][step] += fruits[from..=to]
                    .iter()
                    .map(|row| row[prev_step])
                    .max()
                    .unwrap();
            }
            prev_min_position = cur_min_position;
        }
        diagonal + fruits[last_index][last_index - 1] + fruits[last_index - 1][last_index]
    }
}
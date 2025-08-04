// Last updated: 04.08.2025, 12:31:17
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut fruit1 = fruits[0];
        let mut fruit2_index =
            if let Some(index) = fruits[1..].iter().position(|&fruit| fruit != fruit1) {
                index + 1
            } else {
                return fruits.len() as i32;
            };
        let mut fruit2 = fruits[fruit2_index];
        let mut sequence_start = 0;
        let mut max_len = fruit2_index + 1;
        for (index, &fruit) in fruits.iter().enumerate().skip(fruit2_index + 1) {
            if fruit == fruit2 {
            } else if fruit == fruit1 {
                (fruit1, fruit2, fruit2_index) = (fruit2, fruit1, index);
            } else {
                max_len = max_len.max(index - sequence_start);
                (fruit1, fruit2, fruit2_index, sequence_start) =
                    (fruit2, fruit, index, fruit2_index);
            }
        }
        max_len.max(fruits.len() - sequence_start) as i32
    }
}
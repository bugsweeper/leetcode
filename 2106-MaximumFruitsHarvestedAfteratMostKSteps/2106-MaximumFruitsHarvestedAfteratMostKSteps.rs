// Last updated: 03.08.2025, 16:53:49
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let max_left = start_pos - k;
        let move_right_position = start_pos - k / 3;
        let max_right = start_pos + k;
        let left_start_index = fruits.partition_point(|fruit| fruit[0] < max_left);
        if left_start_index == fruits.len() || fruits[left_start_index][0] > max_right {
            return 0;
        }
        let middle_index = left_start_index
            + fruits[left_start_index..].partition_point(|fruit| fruit[0] <= start_pos);
        let mut right = middle_index;
        let mut sum = fruits[left_start_index..right]
            .iter()
            .map(|fruit| fruit[1])
            .sum();
        if right == fruits.len() {
            return sum;
        }
        let mut max_sum = sum;
        for left in left_start_index..=middle_index {
            let from_position = fruits[left][0];
            let sum_right_til = if from_position < move_right_position {
                // left track is longer, then move left after move right
                start_pos + ((k - start_pos + from_position) >> 1)
            } else if from_position >= start_pos {
                // move only right
                max_right
            } else {
                // right track is longer, then move right after move left
                start_pos + k - ((start_pos - from_position) << 1)
            };
            while fruits[right][0] <= sum_right_til {
                sum += fruits[right][1];
                right += 1;
                if right == fruits.len() {
                    return max_sum.max(sum);
                }
            }
            max_sum = max_sum.max(sum);
            sum -= fruits[left][1];
        }
        max_sum
    }
}
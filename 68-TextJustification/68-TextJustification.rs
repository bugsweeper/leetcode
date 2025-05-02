// Last updated: 02.05.2025, 14:34:15
#[inline]
fn justify_row(words: &mut Vec<String>, cumulative_len: usize, goal_len: usize) -> String {
    if words.len() == 1 {
        let mut row = words.remove(0);
        row.extend((0..goal_len - cumulative_len).map(|_| ' '));
        return row;
    }
    let space_count = words.len() - 1;
    let cumulative_space = goal_len - cumulative_len;
    let space_size = cumulative_space / space_count;
    let bigger_spaces = cumulative_space % space_count;
    let mut row = String::with_capacity(goal_len);
    for (index, word) in words.drain(..space_count).enumerate() {
        row.push_str(&word);
        row.extend((0..space_size + if index < bigger_spaces { 1 } else { 0 }).map(|_| ' '));
    }
    row.push_str(&words.remove(0));
    row
}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::with_capacity(words.len());
        let mut row_words = Vec::with_capacity(max_width / 2);
        let mut cumulative_len = 0;
        for word in words {
            if cumulative_len + row_words.len() + word.len() > max_width {
                result.push(justify_row(&mut row_words, cumulative_len, max_width));
                cumulative_len = 0;
            }
            cumulative_len += word.len();
            row_words.push(word);
        }
        let mut last_row = String::with_capacity(max_width);
        let mut count = row_words.len() - 1;
        for word in row_words.drain(..count) {
            last_row.push_str(&word);
            last_row.push(' ');
        }
        last_row.push_str(&row_words.remove(0));
        count = max_width - last_row.len();
        last_row.extend((0..count).map(|_| ' '));
        result.push(last_row);
        result
    }
}
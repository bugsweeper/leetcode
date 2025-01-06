#[inline]
fn calculate_moves(boxes: &[u8], answer: &mut [i32], iter: impl Iterator<Item = usize>) {
    let mut moves = 0;
    let mut moved_boxes = 0;
    for i in iter {
        moves += moved_boxes;
        *unsafe { answer.get_unchecked_mut(i) } += moves;
        if *unsafe { boxes.get_unchecked(i) } == b'1' {
            moved_boxes += 1;
        }
    }
}

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes_bytes = boxes.as_bytes();
        let mut answer = vec![0; boxes_bytes.len()];
        // forward direction
        calculate_moves(boxes_bytes, &mut answer, 0..boxes.len());
        // backward direction
        calculate_moves(boxes_bytes, &mut answer, (0..boxes.len()).rev());
        answer
    }
}
// Last updated: 03.06.2025, 11:59:14
const BOX_IS_OPEN: i32 = 1;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut status = status;
        // Available but not yet open boxes
        let mut available_boxes = vec![false; status.len()];
        let mut used_boxes = vec![false; status.len()];
        // Queue of boxes to open
        let mut box_queue = Vec::with_capacity(status.len());
        for box_index in initial_boxes {
            let box_index = box_index as usize;
            if status[box_index] == BOX_IS_OPEN {
                box_queue.push(box_index);
                used_boxes[box_index] = true;
            } else {
                available_boxes[box_index] = true;
            }
        }
        let mut total_candies = 0;
        while let Some(box_index) = box_queue.pop() {
            total_candies += candies[box_index];
            for &contained_box in &contained_boxes[box_index] {
                let contained_box = contained_box as usize;
                if used_boxes[contained_box] {
                    continue;
                }
                if status[contained_box] == BOX_IS_OPEN {
                    box_queue.push(contained_box);
                    used_boxes[box_index] = true;
                } else {
                    available_boxes[contained_box] = true;
                }
            }
            for &key in &keys[box_index] {
                let key = key as usize;
                if used_boxes[key] {
                    continue;
                }
                if available_boxes[key] {
                    box_queue.push(key);
                    used_boxes[key] = true;
                } else {
                    // We have key, but not the box itself
                    status[key] = BOX_IS_OPEN;
                }
            }
        }
        total_candies
    }
}
// Last updated: 06.08.2025, 13:49:36
#[derive(PartialEq, Eq)]
enum OperationState {
    ChangedMax(i32),
    KeepMax,
    Unplaced,
}

struct SegmentTree {
    data: Vec<i32>,
}

impl SegmentTree {
    fn new(baskets: Vec<i32>) -> Self {
        let leafs_count = baskets.len();
        let nodes_count = leafs_count - 1;
        let tree_size = nodes_count + leafs_count;
        let mut data = vec![0; tree_size];
        let power_of_two = tree_size.ilog2();
        if (tree_size + 1).ilog2() > power_of_two {
            // All levels are complete, last row is input data
            data[nodes_count..].copy_from_slice(&baskets);
        } else {
            // Last level is incomplete, it is in tail of `data`,
            // but it should contain first peace of baskets (leftmost)
            let complete_levels_size = (1 << power_of_two) - 1;
            let incomplete_level_size = tree_size - complete_levels_size;
            data[complete_levels_size..].copy_from_slice(&baskets[..incomplete_level_size]);
            let last_complete_level_tail_size = leafs_count - incomplete_level_size;
            data[complete_levels_size - last_complete_level_tail_size..complete_levels_size].copy_from_slice(&baskets[incomplete_level_size..]);
        }
        for i in (0..nodes_count).rev() {
            let shift = i << 1;
            data[i] = data[shift + 1].max(data[shift + 2]);
        }
        Self{data}
    }
    
    fn place(&mut self, from_index: usize, fruit: i32) -> OperationState {
        let basket = self.data[from_index];
        if basket < fruit {
            return OperationState::Unplaced;
        }
        let left_index = (from_index << 1) + 1;
        if left_index >= self.data.len() {
            // Basket points to big enough leaf of tree
            self.data[from_index] = 0;
            return OperationState::ChangedMax(0);
        }
        match self.place(left_index, fruit) {
            OperationState::KeepMax => OperationState::KeepMax,
            OperationState::ChangedMax(new_left_max) => {
                let new_max = new_left_max.max(self.data[left_index + 1]);
                self.data[from_index] = new_max;
                OperationState::ChangedMax(new_max)
            }
            OperationState::Unplaced => {
                match self.place(left_index + 1, fruit) {
                    OperationState::KeepMax => OperationState::KeepMax,
                    OperationState::ChangedMax(new_right_max) => {
                        let new_max = new_right_max.max(self.data[left_index]);
                        self.data[from_index] = new_max;
                        OperationState::ChangedMax(new_max)
                    }
                    OperationState::Unplaced => OperationState::Unplaced,
                }
            }
        }
    }
}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut segment_tree = SegmentTree::new(baskets);
        let mut unplaced = 0;
        let mut min_unplaced = i32::MAX;
        for fruit in fruits {
            if fruit >= min_unplaced {
                unplaced += 1;
            } else if segment_tree.place(0, fruit) == OperationState::Unplaced {
                min_unplaced = fruit;
                unplaced += 1;
            }
        }
        unplaced
    }
}

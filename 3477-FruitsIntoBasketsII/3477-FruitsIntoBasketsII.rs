// Last updated: 05.08.2025, 13:59:49
#[derive(PartialEq, Eq)]
enum OperationState {
    ChangedMax(i32),
    KeepMax,
    Unplaced,
}

struct Node {
    max: i32,
    left_max: i32,
    right_max: i32,
    left: SegmentTree,
    right: SegmentTree,
}

#[inline]
fn recalculate_maxes(
    changing_child: &mut i32,
    new_child_max: i32,
    second_child: i32,
    self_max: &mut i32,
) -> OperationState {
    *changing_child = new_child_max;
    let new_max = second_child.max(new_child_max);
    if new_max < *self_max {
        *self_max = new_max;
        OperationState::ChangedMax(new_max)
    } else {
        OperationState::KeepMax
    }
}

impl Node {
    fn new(baskets: &[i32]) -> (Self, i32) {
        let size = baskets.len();
        let left_size = size >> 1;
        let (left, left_max) = SegmentTree::new(&baskets[..left_size]);
        let (right, right_max) = SegmentTree::new(&baskets[left_size..]);
        let max = left_max.max(right_max);
        (
            Self {
                left,
                right,
                max,
                left_max,
                right_max,
            },
            max,
        )
    }
    fn place(&mut self, fruit: i32) -> OperationState {
        if self.max < fruit {
            OperationState::Unplaced
        } else {
            match self.left.place(fruit) {
                OperationState::ChangedMax(new_left_max) => recalculate_maxes(
                    &mut self.left_max,
                    new_left_max,
                    self.right_max,
                    &mut self.max,
                ),
                OperationState::KeepMax => OperationState::KeepMax,
                OperationState::Unplaced => match self.right.place(fruit) {
                    OperationState::ChangedMax(new_right_max) => recalculate_maxes(
                        &mut self.right_max,
                        new_right_max,
                        self.left_max,
                        &mut self.max,
                    ),
                    OperationState::KeepMax => OperationState::KeepMax,
                    OperationState::Unplaced => OperationState::Unplaced,
                },
            }
        }
    }
}

enum SegmentTree {
    Node(Box<Node>),
    Leaf(i32),
}

impl SegmentTree {
    fn new(baskets: &[i32]) -> (Self, i32) {
        let size = baskets.len();
        if size == 1 {
            let max = baskets[0];
            (SegmentTree::Leaf(max), max)
        } else {
            let (node, max) = Node::new(baskets);
            (SegmentTree::Node(Box::new(node)), max)
        }
    }
    fn place(&mut self, fruit: i32) -> OperationState {
        match *self {
            SegmentTree::Node(ref mut node_ref) => node_ref.place(fruit),
            SegmentTree::Leaf(ref mut max) => {
                if *max >= fruit {
                    *max = 0;
                    OperationState::ChangedMax(0)
                } else {
                    OperationState::Unplaced
                }
            }
        }
    }
}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut segment_tree = SegmentTree::new(&baskets).0;
        let mut unplaced = 0;
        for fruit in fruits {
            if segment_tree.place(fruit) == OperationState::Unplaced {
                unplaced += 1;
            }
        }
        unplaced
    }
}
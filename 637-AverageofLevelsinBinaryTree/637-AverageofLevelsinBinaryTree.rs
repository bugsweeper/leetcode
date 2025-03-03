// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

fn fill_level_sum(node: &Option<Rc<RefCell<TreeNode>>>, level_sum: &mut Vec<(f64, u16)>, mut current_level: usize) {
    if node.is_none() {
        return;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    if level_sum.len() > current_level {
        let level_sum = level_sum.get_mut(current_level).unwrap();
        level_sum.0 += node_ref.val as f64;
        level_sum.1 += 1;
    } else {
        level_sum.push((node_ref.val as f64, 1));
    }
    current_level += 1;
    fill_level_sum(&node_ref.left, level_sum, current_level);
    fill_level_sum(&node_ref.right, level_sum, current_level);
}

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut level_sum = Vec::new();
        fill_level_sum(&root, &mut level_sum, 0);
        level_sum.into_iter().map(|(sum, count)| sum / count as f64).collect()
    }
}
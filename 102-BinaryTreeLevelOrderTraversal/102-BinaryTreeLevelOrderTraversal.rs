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

fn fill_level(
    node: &Option<Rc<RefCell<TreeNode>>>,
    level_values: &mut Vec<Vec<i32>>,
    mut level: usize,
) {
    if node.is_none() {
        return;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    if level < level_values.len() {
        level_values[level].push(node_ref.val);
    } else {
        level_values.push(vec![node_ref.val]);
    }
    level += 1;
    fill_level(&node_ref.left, level_values, level);
    fill_level(&node_ref.right, level_values, level);
}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        fill_level(&root, &mut result, 0);
        result
    }
}
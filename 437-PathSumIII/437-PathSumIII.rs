// Last updated: 11.04.2025, 13:09:59
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

fn traverse_path(
    node: &Option<Rc<RefCell<TreeNode>>>,
    target_sum: i64,
    sums: &mut Vec<i64>,
) -> i32 {
    if node.is_none() {
        return 0;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    let last_sum = sums.last().copied().unwrap_or(0) + node_ref.val as i64;
    let mut count = if last_sum == target_sum { 1 } else { 0 };
    for &prev_sum in sums.iter() {
        if last_sum - prev_sum == target_sum {
            count += 1;
        }
    }
    sums.push(last_sum);
    let current_length = sums.len();
    if node_ref.left.is_some() {
        count += traverse_path(&node_ref.left, target_sum, sums);
    }
    if node_ref.right.is_some() {
        sums.truncate(current_length);
        count += traverse_path(&node_ref.right, target_sum, sums);
    }
    count
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut sums = Vec::with_capacity(1000);
        traverse_path(&root, target_sum as i64, &mut sums)
    }
}
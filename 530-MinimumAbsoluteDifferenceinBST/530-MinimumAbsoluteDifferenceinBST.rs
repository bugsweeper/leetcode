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

fn get_values(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    values.push(node_ref.val);
    get_values(&node_ref.left, values);
    get_values(&node_ref.right, values);
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values = Vec::with_capacity(10_000);
        get_values(&root, &mut values);
        values.sort_unstable();
        let mut iter = values.into_iter();
        let mut prev_value = iter.next().unwrap();
        let mut min_diff = i32::MAX;
        for value in iter {
            min_diff = min_diff.min(value - prev_value);
            prev_value = value;
        }
        min_diff
    }
}
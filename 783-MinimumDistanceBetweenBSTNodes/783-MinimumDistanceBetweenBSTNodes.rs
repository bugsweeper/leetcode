// Last updated: 28.04.2025, 14:30:45
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

fn to_vec(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    values.push(node_ref.val);
    to_vec(&node_ref.left, values);
    to_vec(&node_ref.right, values);
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values = Vec::with_capacity(100);
        to_vec(&root, &mut values);
        values.sort_unstable();
        values
            .iter()
            .skip(1)
            .zip(values.iter())
            .map(|(next, prev)| (*next - *prev).abs())
            .min()
            .unwrap() as i32
    }
}
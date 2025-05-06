// Last updated: 06.05.2025, 15:56:32
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

fn fill_leaf_sequence(node: &Option<Rc<RefCell<TreeNode>>>, sequence: &mut Vec<i32>) {
    let node_ref = node.as_ref().unwrap().borrow();
    if node_ref.left.is_none() && node_ref.right.is_none() {
        sequence.push(node_ref.val);
        return;
    }
    if node_ref.left.is_some() {
        fill_leaf_sequence(&node_ref.left, sequence);
    }
    if node_ref.right.is_some() {
        fill_leaf_sequence(&node_ref.right, sequence);
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut s1 = Vec::with_capacity(200);
        let mut s2 = Vec::with_capacity(200);
        fill_leaf_sequence(&root1, &mut s1);
        fill_leaf_sequence(&root2, &mut s2);
        s1 == s2
    }
}
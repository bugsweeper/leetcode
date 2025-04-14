// Last updated: 14.04.2025, 13:13:34
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

fn find_target(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, seen: &mut Vec<bool>) -> bool {
    if node.is_none() {
        return false;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    let pair = k - node_ref.val;
    if pair <= 10000 && pair >= -10000 && seen[(pair + 10000) as usize] {
        return true;
    }
    seen[(node_ref.val + 10000) as usize] = true;
    find_target(&node_ref.left, k, seen) || find_target(&node_ref.right, k, seen)
}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if k > 20000 || k < -20000 {
            return false;
        }
        let mut seen = vec![false; 20000];
        find_target(&root, k, &mut seen)
    }
}

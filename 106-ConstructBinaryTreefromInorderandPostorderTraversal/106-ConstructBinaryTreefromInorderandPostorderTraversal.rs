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

fn build_tree(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() {
        return None;
    }
    let root_value = *postorder.last().unwrap();
    let left_length = inorder.iter().position(|&val| val == root_value).unwrap();
    Some(Rc::new(RefCell::new(TreeNode { val: root_value, left: build_tree(&inorder[0..left_length], &postorder[..left_length]), right: build_tree(&inorder[left_length + 1..], &postorder[left_length..postorder.len() - 1]) })))
}

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_tree(&inorder, &postorder)
    }
}
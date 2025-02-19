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

fn build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    let root_value = preorder[0];
    let left_length = inorder.iter().position(|&val| val == root_value).unwrap();
    let right_start = left_length + 1;
    Some(Rc::new(RefCell::new(TreeNode { val: root_value, left: build_tree(&preorder[1..=left_length], &inorder[..left_length]), right: build_tree(&preorder[right_start..], &inorder[right_start..]) })))
    
}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_tree(&preorder, &inorder)
    }
}
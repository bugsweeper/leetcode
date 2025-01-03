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

fn traverse(node: Option<Rc<RefCell<TreeNode>>>, destination: &mut Vec<i32>) {
    if let Some(node) = node {
        let TreeNode{val, left, right} = Rc::into_inner(node).unwrap().into_inner();
        traverse(left, destination);
        destination.push(val);
        traverse(right, destination);
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer = Vec::new();
        traverse(root, &mut answer);
        answer
    }
}
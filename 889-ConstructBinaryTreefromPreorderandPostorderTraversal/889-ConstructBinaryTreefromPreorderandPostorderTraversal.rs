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

fn construct_from_pre_post(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    match preorder.len() {
        0 => None,
        1 => Some(Rc::new(RefCell::new(TreeNode::new(preorder[0])))),
        _ => {
            let left_root = preorder[1];
            let left_length = postorder
                .iter()
                .position(|&item| item == left_root)
                .unwrap()
                + 1;
            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[0],
                left: construct_from_pre_post(
                    &preorder[1..=left_length],
                    &postorder[..left_length],
                ),
                right: construct_from_pre_post(
                    &preorder[left_length + 1..],
                    &postorder[left_length..postorder.len() - 1],
                ),
            })))
        }
    }
}

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        construct_from_pre_post(&preorder, &postorder)
    }
}
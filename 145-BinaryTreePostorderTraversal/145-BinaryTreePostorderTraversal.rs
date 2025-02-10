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

fn fill_vec(root: &Option<Rc<RefCell<TreeNode>>>, vector: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    let root = root.as_ref().unwrap().borrow();
    fill_vec(&root.left, vector);
    fill_vec(&root.right, vector);
    vector.push(root.val);
}

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        fill_vec(&root, &mut result);
        result
    }
}
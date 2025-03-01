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

fn generate_path_recursive(node: &Option<Rc<RefCell<TreeNode>>>, path: &mut String, result: &mut Vec<String>) {
    let node_ref = node.as_ref().unwrap().borrow();
    path.push_str(&node_ref.val.to_string());
    if node_ref.left.is_none() && node_ref.right.is_none() {
        result.push(path.clone());
        return;
    }
    path.push_str("->");
    let len_with_arrow = path.len();
    if node_ref.left.is_some() {
        generate_path_recursive(&node_ref.left, path, result);
    }
    if node_ref.right.is_some() {
        path.truncate(len_with_arrow);
        generate_path_recursive(&node_ref.right, path, result);
    }
}

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut path = String::new();
        let mut result = Vec::new();
        generate_path_recursive(&root, &mut path, &mut result);
        result
    }
}
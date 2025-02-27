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

fn add_right_nodes(node: &Option<Rc<RefCell<TreeNode>>>, right_nodes: &mut Vec<i32>, mut current_depth: usize) {
    if node.is_none() {
        return;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    if right_nodes.len() <= current_depth {
        right_nodes.push(node_ref.val);
    }
    current_depth += 1;
    add_right_nodes(&node_ref.right, right_nodes, current_depth);
    add_right_nodes(&node_ref.left, right_nodes, current_depth);
}

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(100);
        add_right_nodes(&root, &mut result, 0);
        result
    }
}
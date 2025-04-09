// Last updated: 09.04.2025, 16:59:08
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

fn calc_hash(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    if node.is_none() {
        return 0;
    }
    let node_ref = node.as_ref().unwrap().borrow();
    (calc_hash(&node_ref.left).rotate_left(1) ^ calc_hash(&node_ref.right)).rotate_left(1)
        ^ node_ref.val as usize
}

fn has_subtree(
    node: &Option<Rc<RefCell<TreeNode>>>,
    sub_tree_hash: usize,
    sub_tree: &Option<Rc<RefCell<TreeNode>>>,
) -> (bool, usize) {
    if node.is_none() {
        return (false, 0);
    }
    let node_ref = node.as_ref().unwrap().borrow();
    let (left_has_subtree, left_hash) = has_subtree(&node_ref.left, sub_tree_hash, sub_tree);
    if left_has_subtree {
        return (true, left_hash);
    }
    let (right_has_subtree, right_hash) = has_subtree(&node_ref.right, sub_tree_hash, sub_tree);
    if right_has_subtree {
        return (true, right_hash);
    }
    let hash = (left_hash.rotate_left(1) ^ right_hash).rotate_left(1) ^ node_ref.val as usize;
    (hash == sub_tree_hash && node == sub_tree, hash)
}

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let sub_root_hash = calc_hash(&sub_root);
        has_subtree(&root, sub_root_hash, &sub_root).0
    }
}
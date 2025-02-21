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
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

struct FindElements {
    set: HashSet<i32>,
}

fn fill_set(
    node: &Option<Rc<RefCell<TreeNode>>>,
    x: i32,
    set: &mut std::collections::HashSet<i32>,
) {
    if node.is_none() {
        return;
    }
    let ref_node = node.as_ref().unwrap().borrow();
    set.insert(x);
    fill_set(&ref_node.left, 2 * x + 1, set);
    fill_set(&ref_node.right, 2 * x + 2, set);
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut set = HashSet::new();
        fill_set(&root, 0, &mut set);
        Self { set }
    }

    fn find(&self, target: i32) -> bool {
        self.set.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
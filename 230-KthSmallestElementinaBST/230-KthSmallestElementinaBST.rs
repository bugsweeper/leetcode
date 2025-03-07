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

// Returns Err(<smallest value>) or Ok(<element count>)
// I know it is reversed logic, but it has very easy early return
fn search(node: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> Result<i32, i32> {
    if node.is_none() {
        return Ok(0);
    }
    let node_ref = node.as_ref().unwrap().borrow();
    let left_size = search(&node_ref.left, k)?;
    if left_size == k - 1 {
        return Err(node_ref.val);
    }
    let right_size = search(&node_ref.right, k - left_size - 1)?;
    Ok(left_size + right_size + 1)
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if let Err(answer) = search(&root, k) {
            return answer;
        }
        unimplemented!()
    }
}
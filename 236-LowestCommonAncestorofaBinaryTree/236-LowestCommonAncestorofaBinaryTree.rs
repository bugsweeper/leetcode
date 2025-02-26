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

fn search_lowest_common_ancestor(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return (0, None);
    }
    let root_ref = root.as_deref().unwrap().borrow();
    let left_result = search_lowest_common_ancestor(&root_ref.left, p, q);
    if left_result.0 == 2 {
        return left_result;
    }
    let right_result = search_lowest_common_ancestor(&root_ref.right, p, q);
    if right_result.0 == 2 {
        return right_result;
    }
    let own_count = if root_ref.val == p || root_ref.val == q {
        1
    } else {
        0
    };
    let sum = left_result.0 + right_result.0 + own_count;
    (sum, if sum == 2 { root.clone() } else { None })
}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        search_lowest_common_ancestor(
            &root,
            p.as_deref().unwrap().borrow().val,
            q.as_deref().unwrap().borrow().val,
        )
        .1
    }
}
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

#[derive(Copy, Clone)]
enum KthSmallestResult {
    Found(i32),
    NumNodes(i32),
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        match Self::kth_smallest_impl(&root, k - 1) {
            KthSmallestResult::Found(val) => val,
            KthSmallestResult::NumNodes(_) => unreachable!(),
        }
    }

    fn kth_smallest_impl(root: &Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> KthSmallestResult {
        let Some(root) = root else {
            return KthSmallestResult::NumNodes(0);
        };

        let mut total_nodes = 0;

        let root = root.borrow();
        match Self::kth_smallest_impl(&root.left, k) {
            KthSmallestResult::Found(val) => return KthSmallestResult::Found(val),
            KthSmallestResult::NumNodes(num_nodes) => {
                k -= num_nodes;
                total_nodes += num_nodes;
            },
        }

        if k == 0 {
            return KthSmallestResult::Found(root.val);
        }
        k -= 1;
        total_nodes += 1;

        match Self::kth_smallest_impl(&root.right, k) {
            KthSmallestResult::Found(val) => return KthSmallestResult::Found(val),
            KthSmallestResult::NumNodes(num_nodes) => {
                total_nodes += num_nodes;
            },
        }
        
        KthSmallestResult::NumNodes(total_nodes)
    }
}
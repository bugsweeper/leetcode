// Last updated: 01.04.2025, 11:10:10
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sort_vec = Vec::with_capacity(50_000);
        let mut node_ref = &head;
        while node_ref.is_some() {
            let node_inner = node_ref.as_ref().unwrap();
            sort_vec.push(node_inner.val);
            node_ref = &node_inner.next;
        }
        sort_vec.sort_unstable();
        let mut head = head;
        let mut node_mut = &mut head;
        for val in sort_vec {
            let node_inner = node_mut.as_mut().unwrap();
            node_inner.val = val;
            node_mut = &mut node_inner.next;
        }
        head
    }
}
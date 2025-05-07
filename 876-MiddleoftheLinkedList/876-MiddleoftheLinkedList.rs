// Last updated: 07.05.2025, 12:15:52
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut node_ref = &head;
        while node_ref.is_some() {
            node_ref = &node_ref.as_ref().unwrap().next;
            count += 1;
        }
        let mut head = head;
        let mut node_ref = &mut head;
        for _ in 0..count / 2 {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        node_ref.take()
    }
}
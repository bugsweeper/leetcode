// Last updated: 22.05.2025, 14:40:37
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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut node = &head;
        let mut value = 0;
        while node.is_some() {
            let node_ref = node.as_ref().unwrap();
            value = (value << 1) + node_ref.val;
            node = &node_ref.next;
        }
        value
    }
}
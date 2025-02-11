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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let ref_node = &mut &mut head;
        let reversed_node = &mut &mut None;
        while ref_node.is_some() {
            // reconnect head of input list to `reversed_list`
            std::mem::swap(*reversed_node, &mut ref_node.as_mut().unwrap().next);
            // now `reversed_list` contains input list tail, and reversed list is in input reference => swap them
            std::mem::swap(ref_node, reversed_node);
        }
        reversed_node.clone()
    }
}
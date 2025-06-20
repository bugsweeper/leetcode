// Last updated: 20.06.2025, 15:18:13
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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut list = Vec::with_capacity(100_000);
        let mut node = &head;
        while node.is_some() {
            let node_ptr = node.as_ref().unwrap();
            list.push(node_ptr.val);
            node = &node_ptr.next;
        }
        list
            .iter()
            .zip(list.iter().rev())
            .take(list.len() / 2)
            .map(|(&left, &right)| left + right)
            .max()
            .unwrap()
    }
}
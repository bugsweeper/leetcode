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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let ListNode {
            val: mut prev_val,
            next: node,
        } = head.as_mut().unwrap().as_mut();
        let mut node = node;
        // Make reference movement with no clone
        let ref_node = &mut node;
        loop {
            if ref_node.is_none() {
                break;
            }
            if prev_val == ref_node.as_mut().unwrap().val {
                let mut next_node = ref_node.as_mut().unwrap().next.take();
                std::mem::swap(*ref_node, &mut next_node);
            } else {
                prev_val = ref_node.as_mut().unwrap().val;
                *ref_node = &mut ref_node.as_mut().unwrap().next;
            }
        }
        head
    }
}
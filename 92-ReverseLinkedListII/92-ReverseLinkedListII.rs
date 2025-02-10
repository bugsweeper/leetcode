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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let ref_node = &mut &mut head;
        // Iterate to first disconnection point
        for _ in 1..left {
            *ref_node = &mut ref_node.as_mut().unwrap().next;
        }
        let mut stack = Vec::with_capacity((right - left + 1) as usize);
        for _ in left..=right {
            // Disconnect next node from current
            let mut next_node = ref_node.take();
            // Reconnect node after next node to current
            std::mem::swap(*ref_node, &mut next_node.as_mut().unwrap().next);
            // Save disconnected node
            stack.push(next_node);
        }
        while let Some(mut node) = stack.pop() {
            // Reconnect tail to `node` from stack
            std::mem::swap(*ref_node, &mut node.as_mut().unwrap().next);
            // Connect current node to `node` from stack
            **ref_node = node;
            // Move reference to position after `node` from stack
            *ref_node = &mut ref_node.as_mut().unwrap().next;
        }
        head
    }
}
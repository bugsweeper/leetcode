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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let ref_node1 = &mut &mut l1;
        let ref_node2 = &mut &mut l2;
        let mut carry = 0;
        // Make reference movement with no clone
        loop {
            if ref_node1.is_some() {
                if ref_node2.is_some() {
                    let sum =
                        ref_node1.as_mut().unwrap().val + ref_node2.as_mut().unwrap().val + carry;
                    carry = sum / 10;
                    ref_node1.as_mut().unwrap().val = sum % 10;
                    (*ref_node1, *ref_node2) = (
                        &mut ref_node1.as_mut().unwrap().next,
                        &mut ref_node2.as_mut().unwrap().next,
                    )
                } else if carry > 0 {
                    let sum = ref_node1.as_mut().unwrap().val + carry;
                    carry = sum / 10;
                    ref_node1.as_mut().unwrap().val = sum % 10;
                    *ref_node1 = &mut ref_node1.as_mut().unwrap().next;
                } else {
                    break;
                }
            } else {
                if ref_node2.is_some() {
                    std::mem::swap(*ref_node1, *ref_node2);
                    continue;
                } else {
                    if carry > 0 {
                        **ref_node1 = Some(Box::new(ListNode::new(carry)));
                    }
                    break;
                }
            }
        }
        l1
    }
}
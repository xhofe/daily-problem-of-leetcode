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
    pub fn add_two_numbers_rev(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
            carry: i32,
        ) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (None, None) => {
                    if carry == 0 {
                        None
                    } else {
                        Some(Box::new(ListNode::new(carry)))
                    }
                }
                (Some(l1), None) => {
                    let sum = l1.val + carry;
                    let carry = sum / 10;
                    let val = sum % 10;
                    Some(Box::new(ListNode {
                        val,
                        next: add_two_numbers(l1.next, None, carry),
                    }))
                }
                (None, Some(l2)) => {
                    let sum = l2.val + carry;
                    let carry = sum / 10;
                    let val = sum % 10;
                    Some(Box::new(ListNode {
                        val,
                        next: add_two_numbers(None, l2.next, carry),
                    }))
                }
                (Some(l1), Some(l2)) => {
                    let sum = l1.val + l2.val + carry;
                    let carry = sum / 10;
                    let val = sum % 10;
                    Some(Box::new(ListNode {
                        val,
                        next: add_two_numbers(l1.next, l2.next, carry),
                    }))
                }
            }
        }
        add_two_numbers(l1, l2, 0)
    }
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1 = Self::reverse_list(l1);
        let l2 = Self::reverse_list(l2);
        let sum = Self::add_two_numbers_rev(l1, l2);
        Self::reverse_list(sum)
    }
}

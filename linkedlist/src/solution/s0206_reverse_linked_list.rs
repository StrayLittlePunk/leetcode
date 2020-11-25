#![allow(unused)]
pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
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
        if head == None {
            return None;
        }
        let mut current = None;
        let mut next = head;
        while let Some(mut boxed_node) = next.take() {
            let next_node = boxed_node.next.take();
            boxed_node.next = current.take();

            current = Some(boxed_node);
            next = next_node;
        }
        current
    }
    pub fn reverse_list_r(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //递归终止条件是当前为空，或者下一个节点为空
        fn rev(cur: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut p) = cur {
                let next = p.next;
                p.next = prev;
                return rev(next, Some(p));
            } else {
                prev
            }
        }
        rev(head, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_206() {
        assert_eq!(
            Solution::reverse_list(to_list(vec![1, 2, 3, 4, 5])),
            to_list(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_list_r(to_list(vec![1, 2, 3, 4, 5])),
            to_list(vec![5, 4, 3, 2, 1])
        )
    }
}

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
    #[inline]
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut p = &mut result;

        while l1.is_some() && l2.is_some() {
            let deref_val = |node: &Box<ListNode>| (*node).val;
            let digit1 = l1.as_ref().map_or(0, deref_val);
            let digit2 = l2.as_ref().map_or(0, deref_val);
            if digit1 <= digit2 {
                let tmp = l1.as_mut().unwrap().next.take();
                p.next = l1.take();
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                p.next = l2.take();
                l2 = tmp;
            }
            p = p.next.as_mut().unwrap();

            /*             let deref_next = |node: Box<ListNode>| (*node).next;
             *
             *             l1 = l1.and_then(deref_next);
             *             l2 = l2.and_then(deref_next); */
        }
        // 如果l1 是None，则把l2 剩余节点接到结果上，反之亦然，不用考虑两个都为None情况
        if l1.is_none() {
            p.next = l2.take();
        } else {
            p.next = l1.take();
        }
        result.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4, 5, 6]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4, 7, 8])),
            to_list(vec![1, 1, 2, 3, 4, 4, 7, 8])
        );
    }
}

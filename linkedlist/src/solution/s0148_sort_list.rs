#![allow(unused)]
pub struct Solution {}

/* Author: Saralee
 * URL: https://leetcode.com/problems/sort-list/description/ */

// use std::cmp::Ordering;
use crate::util::linked_list::{to_list, ListNode};
// Definition for singly-linked list.
/* #[derive(PartialEq, Eq, Clone, Debug)]
 * pub struct ListNode {
 *   pub val: i32,
 *   pub next: Option<Box<ListNode>>
 * }
 *
 *
 * impl ListNode {
 *   #[inline]
 *   fn new(val: i32) -> Self {
 *     ListNode {
 *       next: None,
 *       val
 *     }
 *   }
 * } */

// merge sort
impl Solution {
    #[inline]
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::length(&head);

        // base condition
        if len < 2 {
          return head;
        }
        let mut ptr = head.as_mut();
        for _ in 0..(len / 2)  - 1{
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        let (mut right, mut left) = (ptr.unwrap().next.take(), head);

        /* if left.is_some() && right.is_some() &&
         *   left.as_ref().unwrap() != right.as_ref().unwrap() {
         *     left = Self::sort_list(left);
         *     right = Self::sort_list(right);
         *     Self::merge_two_lists(left, right)
         * } else {
         *     Self::merge_two_lists(left, right)
         * } */
         Self::merge_two_lists(Self::sort_list(left), Self::sort_list(right))
            
    }
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }
        fn merge_two_lists(
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
      if l1.is_none() {
          p.next = l2.take();
        }else {
          p.next = l1.take();
        }
        result.next
    }
}

// error: (signal: 11, SIGSEGV: invalid memory reference)

/* impl Solution {
 *     pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
 *         let (lhs, rhs) = Self::split(head);
 *         match (lhs, rhs) {
 *             (None, None) => None,
 *             (lhs, None) => lhs,
 *             (None, rhs) => rhs,
 *             (lhs, rhs) => Self::merge(Self::sort_list(lhs), Self::sort_list(rhs)),
 *         }
 *     }
 *
 *     fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
 *         let mut lhs = None;
 *         let mut rhs = None;
 *         let mut i = true;
 *         loop {
 *             head = match head {
 *                 None => break,
 *                 Some(mut headnode) => {
 *                     let head = headnode.next.take();
 *                     match i {
 *                         true => {
 *                             headnode.next = lhs.take();
 *                             lhs = Some(headnode);
 *                         }
 *                         false => {
 *                             headnode.next = rhs.take();
 *                             rhs = Some(headnode);
 *                         }
 *                     }
 *                     head
 *                 }
 *             };
 *             i = !i;
 *         }
 *         (lhs, rhs)
 *     }
 *     fn merge(
 *         mut l1: Option<Box<ListNode>>,
 *         mut l2: Option<Box<ListNode>>,
 *     ) -> Option<Box<ListNode>> {
 *         let mut result = ListNode::new(0);
 *         let mut p = &mut result;
 *
 *         while l1.is_some() && l2.is_some() {
 *             let deref_val = |node: &Box<ListNode>| (*node).val;
 *             let digit1 = l1.as_ref().map_or(0, deref_val);
 *             let digit2 = l2.as_ref().map_or(0, deref_val);
 *             if digit1 <= digit2 {
 *                 let tmp = l1.as_mut().unwrap().next.take();
 *                 p.next = l1.take();
 *                 l1 = tmp;
 *             } else {
 *                 let tmp = l2.as_mut().unwrap().next.take();
 *                 p.next = l2.take();
 *                 l2 = tmp;
 *             }
 *             p = p.next.as_mut().unwrap();
 *         }
 *         if l1.is_none() {
 *             p.next = l2.take();
 *         } else {
 *             p.next = l1.take();
 *         }
 *         result.next
 *     }
 *
 * } */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_148() {
        let example1 = to_list(vec![4, 2, 1, 3]);
        // example1 moved to sort_list inner
        // because Box type does not implement the `Copy` trait
        // so, if implement the `Copy` trait ,then will copy to function
        assert_eq!(Solution::sort_list(example1), to_list(vec![1, 2, 3, 4]));
        let example2 = to_list(vec![-1, 5, 3, 4, 0]);
        assert_eq!(Solution::sort_list(example2), to_list(vec![-1, 0, 3, 4, 5]));
    }
}

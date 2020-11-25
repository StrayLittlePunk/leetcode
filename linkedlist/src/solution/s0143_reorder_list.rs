#![allow(unused)]
pub struct Solution {}

/* Author: Saralee
 * URL: https://leetcode.com/problems/reorder-list/description/ */

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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // find the middle of linked list [Problem 876]
        // in 1->2->3->4->5->6 find 4
        let len = Self::length(&head);

        let mut ptr = head.as_mut();
        for _ in 0..(len / 2) {
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        if let Some(node) = ptr {
            // reverse the second part of the list [Problem 206]
            // convert 1->2->3->4->5->6 into 1->2->3->4 and 6->5->4
            // reverse the second half in-place
            let reverse = Self::reverse(node.next.take(), None);

            // merge two sorted linked lists [Problem 21]
            // merge 1->2->3->4 and 6->5->4 into 1->6->2->5->3->4
            if let Some(node) = head {
                node.next = Self::merge(reverse, node.next.take(), len - 1);
            }
        }
    }

    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }
    fn reverse(
        head: Option<Box<ListNode>>,
        accumulator: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => accumulator,
            Some(mut node) => {
                let next = node.next;
                node.next = accumulator;

                Self::reverse(next, Some(node))
            }
        }
    }

    fn merge(
        mut left: Option<Box<ListNode>>,
        right: Option<Box<ListNode>>,
        count: usize,
    ) -> Option<Box<ListNode>> {
        if count == 0 {
            None
        } else {
            match (left.as_mut(), right.as_ref()) {
                (None, None) => None,
                (Some(_), None) => left,
                (None, Some(_)) => right,
                (Some(l), Some(_)) => {
                    l.next = Self::merge(right, l.next.take(), count - 1);
                    left
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_143() {
        let mut example1 = to_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut example1);
        assert_eq!(example1, to_list(vec![1, 4, 2, 3]));

        let mut example2 = to_list(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut example2);
        assert_eq!(example2, to_list(vec![1, 5, 2, 4, 3]));
    }
}

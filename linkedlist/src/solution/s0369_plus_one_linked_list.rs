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
    pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack: Vec<Box<ListNode>> = Vec::new();
        while head.is_some() {
            let tmp = head.as_mut().unwrap().next.take();
            stack.push(head.take().unwrap());
            head = tmp;
        }
        // let mut dummy = Box::new(ListNode{val: -1, next: head});
        //let mut p = &mut dummy.next;
        let mut p = None;
        let mut val = 1;
        while let Some(mut node) = stack.pop() {
            if node.val + val == 10 {
                val = 1;
                node.val = 0;
            } else {
                node.val = node.val + val;
                val = 0;
            }
            node.next = p;
            p = Some(node);
        }
        if val == 1 {
            Some(Box::new(ListNode { val: 1, next: p }))
        } else {
            p
        }
    }
/*     pub fn plus_one_best(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
 *         let mut sentinel = Box::new(ListNode { val: 0, next: head });
 *         let mut not_nine = &mut sentinel;
 *         let mut p = sentinel.next.as_mut();
 *
 *         while let Some(node) = p {
 *             if node.val != 9 {
 *                 not_nine = node;
 *             }
 *             p = node.next.as_ref();
 *         }
 *
 *
 *         if sentinel.val == 0 {
 *             sentinel.next
 *         } else {
 *             Some(sentinel)
 *         }
 *     } */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_369() {
        assert_eq!(
            Solution::plus_one(to_list(vec![1, 2, 3])),
            to_list(vec![1, 2, 4])
        );
        assert_eq!(
            Solution::plus_one(to_list(vec![1, 2, 9, 4])),
            to_list(vec![1, 2, 9, 5])
        );
        assert_eq!(
            Solution::plus_one(to_list(vec![1, 2, 9])),
            to_list(vec![1, 3, 0])
        );
        assert_eq!(
            Solution::plus_one(to_list(vec![9, 9])),
            to_list(vec![1, 0, 0])
        );
        assert_eq!(Solution::plus_one(to_list(vec![0])), to_list(vec![1]));
    }
}

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
      if head.is_none() {
        return head;
      }
      let len = Self::length(&head);
      let mut n = k % len as i32;
      if len < 2 || n == 0{
        return head;
      }
      let mut ptr = head.as_mut().unwrap();

      let mut r = len as i32 - n;
      while r != 1 {
        ptr = ptr.next.as_mut().unwrap();
        r -= 1;
      }
      let mut tail = ptr.next.take();
      ptr = tail.as_mut().unwrap();
      while n != 1 {
        ptr = ptr.next.as_mut().unwrap();
        n -= 1;
      }
      ptr.next = head;
      tail

    }
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_61() {
        // Explanation:
        // rotate 1 steps to the right: 5->1->2->3->4->NULL
        // rotate 2 steps to the right: 4->5->1->2->3->NULL
        assert_eq!(
            Solution::rotate_right(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![4, 5, 1, 2, 3])
        );
        //Explanation:
        // rotate 1 steps to the right: 2->0->1->NULL
        // rotate 2 steps to the right: 1->2->0->NULL
        // rotate 3 steps to the right: 0->1->2->NULL
        // rotate 4 steps to the right: 2->0->1->NULL
        assert_eq!(
            Solution::rotate_right(to_list(vec![0, 1, 2]), 4),
            to_list(vec![2, 0, 1])
        );
    }
}

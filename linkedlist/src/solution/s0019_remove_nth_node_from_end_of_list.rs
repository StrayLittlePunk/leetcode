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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        unsafe {
            let mut head = head;
            let mut front: *mut Option<Box<ListNode>> = &mut head;
            let mut tail: *mut Option<Box<ListNode>> = &mut head;
            for _ in 0..n {
                front = &mut (*front).as_mut().unwrap().next;
            }
            if (*front).is_none() {
                return head.take().unwrap().next;
            }
            loop {
                front = &mut (*front).as_mut().unwrap().next;
                if (*front).is_none() {
                    break;
                }
                tail = &mut (*tail).as_mut().unwrap().next;
            }
            (*tail).as_mut().unwrap().next =
                (*tail).as_mut().unwrap().next.as_mut().unwrap().next.take();
            head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
    }
}

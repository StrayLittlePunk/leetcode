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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut rev: Option<Box<ListNode>> = None;
        let mut _nxt: Option<Box<ListNode>> = None;

        // half to half
        while head.is_some() {
            if rev == head || rev == head.as_ref().unwrap().next {
                return true;
            }

            _nxt = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = rev;
            rev = head;
            head = _nxt;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_234() {
        assert_eq!(Solution::is_palindrome(to_list(vec![1, 2, 4])), false);
        assert_eq!(Solution::is_palindrome(to_list(vec![1, 2, 2, 1])), true);
    }
}

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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
      let mut p = head.as_mut();
      while let Some(t) = p {
        if t.val == val {
          head = t.next.take();
          p = head.as_mut();
        } else if t.next.is_some() && val == t.next.as_ref().unwrap().val {
          let tmp = t.next.as_mut().unwrap().next.take();
          t.next = tmp;
          p = Some(t);
        }else {
         p = t.next.as_mut();
        } 
      }
      head
    }

     pub fn remove_elements_a(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;

        loop {
            match ptr {
                None => break,
                Some(node) if node.val == val => {
                    *ptr = node.next.take();
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_203() {
        assert_eq!(
            Solution::remove_elements(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            to_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(Solution::remove_elements(to_list(vec![1]), 1), None);
        assert_eq!(
            Solution::remove_elements_a(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            to_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(Solution::remove_elements_a(to_list(vec![1]), 1), None); }
}

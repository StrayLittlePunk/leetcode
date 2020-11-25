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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut p = head.as_mut();
      while let Some(t) = p {
        if !t.next.as_ref().is_none() && t.val == t.next.as_ref().unwrap().val {
          let tmp = t.next.as_mut().unwrap().next.take();
          t.next = tmp;
          p = Some(t);
        }else {
         p = t.next.as_mut();
        } 
      }
      head
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_83() {
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 2])),
            to_list(vec![1, 2])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 2, 3, 3])),
            to_list(vec![1, 2, 3])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 1])),
            to_list(vec![1])
        );
    }
}
